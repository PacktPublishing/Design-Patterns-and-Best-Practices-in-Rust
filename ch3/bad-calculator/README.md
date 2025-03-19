# Bad Calculator - Chapter 3

This directory contains examples from Chapter 3: "Anti-Pattern: Using Clone & Rc Everywhere". Each example demonstrates both the anti-pattern and a better alternative.

## Examples

1. **Avoiding Ownership Concerns**
   - Demonstrates the problems with trying to avoid ownership thinking
   - Shows a better approach that respects Rust's ownership model

2. **Cloning Everything**
   - Shows the inefficiency of using clone() to avoid ownership issues
   - Provides a better approach using references and proper lifetimes

3. **Misusing Rc and RefCell**
   - Illustrates overuse of smart pointers to avoid ownership thinking
   - Presents a cleaner alternative with explicit ownership

## Running the Examples

Each example can be run separately:

```bash
cargo run --bin ownership-concerns
cargo run --bin cloning-everything
cargo run --bin rc-and-refcell
```

These examples demonstrate anti-patterns and their solutions, building on the "Bad Calculator" concept from previous chapters.
