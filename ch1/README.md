# Chapter 1: Why is Rust Different?

This directory contains code examples from Chapter 1 of the book. In this chapter, we introduce why Rust requires different design patterns and ways of thinking.

## Contents

1. **bad_calculator**: The initial setup for our "Bad Calculator" project
   - Basic framework for a calculator application
   - Demonstrates the initial skeleton that we'll build on in future chapters

2. **block_finder.rs**: A standalone example that demonstrates ownership issues
   - Shows how ownership and borrowing constraints can affect design
   - Example of attempting to use a Rust-unfriendly design pattern

## Running the Examples

To run the bad_calculator example:

```bash
cd bad_calculator
cargo run
```

The block_finder.rs file is primarily for illustration and isn't intended to be run directly as it demonstrates issues with designs that don't align with Rust's ownership model.

These examples serve as a foundation for understanding why Rust requires different approaches to design, which we'll explore in later chapters.
