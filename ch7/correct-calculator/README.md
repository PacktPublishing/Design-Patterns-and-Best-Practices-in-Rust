# Correct Calculator - Chapter 7

This project demonstrates behavioral design patterns in Rust, building on the structural patterns from Chapter 6 to create a more interactive and flexible calculator.

## Patterns Implemented

### Structural Patterns (from Chapter 6)
1. **Composite Pattern**: Hierarchical expression trees that respect operator precedence
2. **Adapter Pattern**: Connecting calculator components with different interfaces
3. **Bridge Pattern**: Separating abstraction from implementation for display and evaluation

### Behavioral Patterns (Chapter 7 focus)
1. **Command Pattern**: Encapsulates calculator operations as objects
2. **Chain of Responsibility**: Creates a flexible input processing pipeline
3. **Strategy Pattern**: Implements different evaluation algorithms
4. **Mediator Pattern**: Coordinates between calculator components
5. **Template Method**: Structures algorithms while allowing specific steps to be overridden

## Key Features
- Expression evaluation with proper operator precedence
- Command history with undo/redo capability
- Variable storage and manipulation
- Pluggable evaluation strategies
- Flexible input handling

## Running the Project

```
cargo run
```

When running, try the following commands:
- Expressions like `2 + 3 * 4`
- Variable assignment like `x = 5`
- Commands like `/undo`, `/redo`, `/history`
- `/demo_ch6` to see Chapter 6 structural patterns in action

This version builds on Chapter 6 and will be further enhanced in Chapter 8.
