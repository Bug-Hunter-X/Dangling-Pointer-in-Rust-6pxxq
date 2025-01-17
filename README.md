# Dangling Pointer in Rust

This repository demonstrates a common error in Rust involving dangling pointers. Dangling pointers arise when you try to access memory after the object it points to has been dropped.  This leads to undefined behavior (UB) in Rust, making your program unpredictable and potentially crashing.

The code in `dangling_pointer.rs` shows how this can happen, and `dangling_pointer_solution.rs` demonstrates a safe alternative.

## How to Run

1. Clone this repository.
2. Navigate to the directory.
3. Compile and run using `cargo run`.

## Learning Points

* Avoid raw pointers (`*mut T`, `*const T`) whenever possible.  Use safe abstractions like references (`&T`, `&mut T`) instead.
* Be mindful of memory lifetimes. Ensure that pointers are only used while the data they point to is still valid.
* Understand how `drop()` affects memory management in Rust.