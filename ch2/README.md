# Chapter 2: Anti-Pattern: Designing for Object Orientation

This directory contains code examples from Chapter 2 of the book, which explores the anti-pattern of trying to force object-oriented designs in Rust.

## Contents

1. **bad_calculator**: Continuation of our Bad Calculator project
   - Demonstrates attempting to use OO inheritance-like patterns in Rust
   - Shows why these approaches create problems

2. **not_so_bad_calculator**: A version that uses more Rust-appropriate patterns
   - Shows how to implement similar functionality in a more idiomatic way
   - Uses Rust's enums and pattern matching instead of class hierarchies

3. **worse_calculator**: An even more problematic OO-style implementation
   - Demonstrates how layering OO abstractions creates complexity
   - Shows why deep hierarchies are problematic in Rust

4. **pets**: A simple example illustrating limitations of trait-based polymorphism
   - Demonstrates the differences between Rust traits and OO interfaces

5. **block_finder.rs**: Another example of OO design issues

## Running the Examples

Each example can be built and run with cargo:

```bash
cd bad_calculator
cargo run
```

```bash
cd not_so_bad_calculator
cargo run
```

```bash
cd worse_calculator
cargo run
```

```bash
cd pets
cargo run
```

These examples illustrate the challenges and problems that arise when trying to force object-oriented design patterns into Rust code, and begin to show more idiomatic alternatives.
