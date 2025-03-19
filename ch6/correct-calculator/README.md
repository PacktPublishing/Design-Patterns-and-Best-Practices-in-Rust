# Correct Calculator - Chapter 6

This project demonstrates the implementation of structural design patterns in Rust through a calculator application.

## Patterns Implemented

- **Composite Pattern**: Expression trees for handling operator precedence
- **Decorator Pattern**: Adding functionality to expressions (logging, timing)
- **Adapter Pattern**: Connecting different calculator modes and services
- **Facade Pattern**: Simplified calculator interface
- **Bridge Pattern**: Separating expression interface from implementation

## Running the Project

```
cargo run
```

## Structure

- `expression.rs`: Composite pattern for expression trees
- `decorator.rs`: Expression decorators
- `adapter.rs`: Adapters for different calculation services
- `facade.rs`: Simplified calculator interface
- `bridge.rs`: Expression implementation separation

This project builds on the foundation established in Chapter 5, adding structural patterns to enhance the calculator's capabilities.
