//! Resource management using RAII patterns
//!
//! This module demonstrates:
//! - Connection pools with automatic cleanup
//! - Scoped locks with guards
//! - Transaction guards for data consistency
//! - Cascading cleanup on Drop

use crate::error::{SamsaError, Result};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// A connection to the broker (placeholder for actual connection)
#[derive(Debug)]
pub struct Connection {
    last_used: Instant,
    is_healthy: bool,
}

impl Connection {
    fn new(_id: usize) -> Self {
        Self {
            last_used: Instant::now(),
            is_healthy: true,
        }
    }

    pub fn is_healthy(&self) -> bool {
        self.is_healthy && self.last_used.elapsed() < Duration::from_secs(300)
    }

    fn mark_used(&mut self) {
        self.last_used = Instant::now();
    }
}

/// Connection pool with automatic resource management
pub struct ConnectionPool {
    connections: Arc<Mutex<VecDeque<Connection>>>,
    max_connections: usize,
    next_id: Arc<Mutex<usize>>,
}

impl ConnectionPool {
    pub fn new(max_connections: usize) -> Arc<Self> {
        Arc::new(Self {
            connections: Arc::new(Mutex::new(VecDeque::new())),
            max_connections,
            next_id: Arc::new(Mutex::new(0)),
        })
    }

    /// Acquire a connection from the pool
    ///
    /// Returns a ConnectionGuard that automatically returns the connection
    /// to the pool when dropped (RAII pattern)
    pub fn acquire(self: &Arc<Self>) -> Result<ConnectionGuard> {
        let mut connections = self.connections.lock()
            .map_err(|_| SamsaError::resource("Lock poisoned"))?;

        // Try to reuse an existing connection
        if let Some(mut conn) = connections.pop_front() {
            if conn.is_healthy() {
                conn.mark_used();
                return Ok(ConnectionGuard {
                    connection: Some(conn),
                    pool: self.clone(),
                    acquired_at: Instant::now(),
                });
            }
        }

        // Create a new connection if under limit
        let mut next_id = self.next_id.lock()
            .map_err(|_| SamsaError::resource("Lock poisoned"))?;

        if *next_id < self.max_connections {
            let conn = Connection::new(*next_id);
            *next_id += 1;

            Ok(ConnectionGuard {
                connection: Some(conn),
                pool: self.clone(),
                acquired_at: Instant::now(),
            })
        } else {
            Err(SamsaError::resource("Connection pool exhausted"))
        }
    }

    /// Return a connection to the pool
    fn return_connection(&self, connection: Connection) {
        if let Ok(mut connections) = self.connections.lock() {
            if connection.is_healthy() && connections.len() < self.max_connections {
                connections.push_back(connection);
            }
        }
    }
}

/// RAII guard for automatic connection return
///
/// Demonstrates the RAII pattern: the connection is automatically
/// returned to the pool when this guard is dropped
pub struct ConnectionGuard {
    connection: Option<Connection>,
    pool: Arc<ConnectionPool>,
    acquired_at: Instant,
}

impl ConnectionGuard {
    /// Get a reference to the underlying connection
    pub fn connection(&self) -> Option<&Connection> {
        self.connection.as_ref()
    }

    /// Get the duration this connection has been held
    pub fn held_duration(&self) -> Duration {
        self.acquired_at.elapsed()
    }
}

impl Drop for ConnectionGuard {
    fn drop(&mut self) {
        if let Some(connection) = self.connection.take() {
            // Record metrics about connection usage
            let usage_duration = self.acquired_at.elapsed();
            if usage_duration > Duration::from_secs(10) {
                eprintln!("Warning: Connection held for {:?}", usage_duration);
            }

            // Return connection to pool if it's still healthy
            if connection.is_healthy() {
                self.pool.return_connection(connection);
            }
        }
    }
}

/// Transaction guard ensuring data consistency
///
/// Demonstrates RAII for transactions: automatically rolls back
/// if commit() is not called before drop
type RollbackFn<'a, T> = Box<dyn FnOnce(&mut T) + 'a>;

pub struct TransactionGuard<'a, T> {
    data: &'a mut T,
    committed: bool,
    rollback_fn: Option<RollbackFn<'a, T>>,
}

impl<'a, T> TransactionGuard<'a, T> {
    pub fn begin(data: &'a mut T, rollback_fn: RollbackFn<'a, T>) -> Self {
        Self {
            data,
            committed: false,
            rollback_fn: Some(rollback_fn),
        }
    }

    /// Access the data within the transaction
    pub fn data_mut(&mut self) -> &mut T {
        self.data
    }

    /// Commit the transaction
    pub fn commit(mut self) {
        self.committed = true;
        // Transaction is committed, rollback won't happen on drop
    }
}

impl<'a, T> Drop for TransactionGuard<'a, T> {
    fn drop(&mut self) {
        if !self.committed {
            if let Some(rollback) = self.rollback_fn.take() {
                rollback(self.data);
            }
        }
    }
}

/// Scoped lock guard with timeout
///
/// Demonstrates RAII with additional timeout logic
pub struct TimedLockGuard<'a, T> {
    guard: Option<MutexGuard<'a, T>>,
    acquired_at: Instant,
}

impl<'a, T> TimedLockGuard<'a, T> {
    /// Try to acquire a lock with timeout
    pub fn try_acquire(mutex: &'a Mutex<T>, timeout: Duration) -> Result<Self> {
        let start = Instant::now();

        loop {
            match mutex.try_lock() {
                Ok(guard) => {
                    return Ok(TimedLockGuard {
                        guard: Some(guard),
                        acquired_at: Instant::now(),
                    });
                }
                Err(_) if start.elapsed() < timeout => {
                    std::thread::sleep(Duration::from_millis(1));
                }
                Err(_) => return Err(SamsaError::resource("Lock acquisition timeout")),
            }
        }
    }

    /// Get the duration this lock has been held
    pub fn held_duration(&self) -> Duration {
        self.acquired_at.elapsed()
    }
}

impl<'a, T> std::ops::Deref for TimedLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().unwrap()
    }
}

impl<'a, T> std::ops::DerefMut for TimedLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_pool() {
        let pool = ConnectionPool::new(5);

        // Acquire multiple connections
        let guard1 = pool.acquire().unwrap();
        let guard2 = pool.acquire().unwrap();

        assert!(guard1.connection().is_some());
        assert!(guard2.connection().is_some());

        // Connections are automatically returned when guards drop
        drop(guard1);
        drop(guard2);

        // Can acquire again
        let _guard3 = pool.acquire().unwrap();
    }

    #[test]
    fn test_pool_exhaustion() {
        let pool = ConnectionPool::new(2);

        let _guard1 = pool.acquire().unwrap();
        let _guard2 = pool.acquire().unwrap();

        // Pool is exhausted
        assert!(pool.acquire().is_err());
    }

    #[test]
    fn test_transaction_guard_commit() {
        let mut data = vec![1, 2, 3];

        {
            let mut transaction = TransactionGuard::begin(
                &mut data,
                Box::new(|d| *d = vec![1, 2, 3])
            );

            transaction.data_mut().push(4);
            transaction.commit(); // Explicitly commit
        }

        assert_eq!(data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_transaction_guard_rollback() {
        let mut data = vec![1, 2, 3];

        {
            let mut transaction = TransactionGuard::begin(
                &mut data,
                Box::new(|d| *d = vec![1, 2, 3])
            );

            transaction.data_mut().push(4);
            // Don't commit - should roll back
        }

        assert_eq!(data, vec![1, 2, 3]); // Rolled back
    }
}
