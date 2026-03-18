# Rust Procedural Macros

This repository contains a collection of Rust procedural macro implementations developed for educational purposes. The primary goal of this project is to explore the mechanics of the syn and quote crates for static code generation.

## Project Structure

The codebase is organized into multiple independent modules, each focusing on a specific procedural macro implementation.

- json-macro: A custom derive macro for JSON serialization.
- diff-macro: A planned implementation for structural diffing.

## JSON Macro Implementation

The json-macro module provides a ToJson derive macro that automatically implements the JsonSerialize trait for structs with named fields.

### Components

1. traits: Defines the JsonSerialize trait, which specifies the to_json method.
2. macros: Contains the procedural macro logic using syn and quote.
3. app: A demonstration package showing how to apply the macro to a struct.

## Development Commands

Run individual projects:

```bash
make run-diff
make run-json
make run-map
```

### Usage Example

The following example demonstrates the application of the ToJson macro:

```rust
use macros::ToJson;
use traits::JsonSerialize;

#[derive(ToJson)]
struct Person {
    id: i32,
    age: i32,
    name: String,
}

fn main() {
    let person = Person {
        id: 1,
        age: 20,
        name: String::from("neha"),
    };
    println!("{}", person.to_json());
}
```

## Technical Details

The macros utilize the following core technologies:

- syn: Used for parsing Rust code into an Abstract Syntax Tree (AST).
- quote: Used for turning AST nodes back into Rust source code.
- proc_macro: The standard library crate for procedural macros.

## Development Status

This project is actively being used for learning Rust meta programming concepts. Future additions will include more complex procedural macros such as attribute macros and function like macros.
