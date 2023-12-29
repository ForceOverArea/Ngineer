# gmatlib - Grant's Matrix Algebra Library

Defines a `Matrix<T>` struct for numeric types and implements several useful functions for matrix algebra purposes.
The source here contains a number of `extern "C"` functions for exposing the types and functions to other languages
as well.

The `Matrix<T>` type is simple, consisting of one contiguous piece of memory to reduce indirection and reduce
the number of allocations needed to construct the matrix.

# Example
```rust
use gmatlib::{Matrix, row_vec};

// Create a matrix with 3 columns
let a: Matrix<i32> = Matrix::from_vec(
    3, 
    vec![1, 2, 3,
         4, 5, 6,
         7, 8, 9]
).unwrap();

// Matrices support appropriate binary operations
let b = row_vec![0_i32, 1_i32, 0_i32] * (a * 3);

// ...and concise indexing 
assert_eq!(
    b[(0, 1)],
    15
);
```
