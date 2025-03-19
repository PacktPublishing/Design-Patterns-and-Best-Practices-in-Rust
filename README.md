# Design Patterns and Best Practices in Rust

This repository contains the code examples for the book "Design Patterns and Best Practices in Rust" by Evan Williams.

## Repository Structure

The repository is organized by chapter, with each chapter's examples in its corresponding directory:

- ch1 - Why is Rust Different?
- ch2 - Anti-Pattern: Designing for Object Orientation
- ch3 - Anti-Pattern: Using Clone & Rc<RefCell> Everywhere
- ch4 - Don't Fight the Borrow Checker
- ch5-ch8 - Correct Calculator Project
  - ch5 - Creational Patterns: Making Things
  - ch6 - Structural Patterns: Connecting & Aggregating
  - ch7 - Behavioural Patterns 1: Taking Action
  - ch8 - Behavioural Patterns 2: Keeping Track
- ch9 - Architectural Patterns
- ch10 - Patterns that Leverage the Type System
- ch11 - Patterns from Functional Programming
- ch12 - Patterns that Use Unique Rust Features
- ch13 - Leaning into Rust

## The Correct Calculator Project

The "Correct Calculator" is a project that spans chapters 5-8, demonstrating how to apply various design patterns in Rust. The project evolves across these chapters:

- **Chapter 5 (Creational Patterns)**: Sets up the basic structure and implements patterns for creating objects
- **Chapter 6 (Structural Patterns)**: Enhances the calculator with patterns for organizing code structure
- **Chapter 7 (Behavioral Patterns 1)**: Implements patterns for command processing and actions
- **Chapter 8 (Behavioral Patterns 2)**: Completes the calculator with state management and optimization

To build any chapter's version of the calculator:

```bash
cd ch5/correct-calculator  # or ch6, ch7, ch8
cargo build
cargo run
```

Each chapter builds on the previous one, adding new features while demonstrating new patterns.

## Requirements

- Rust 1.78.0 or newer
- Cargo (included with Rust)

## License

This project is licensed under the terms specified in the LICENSE file.
