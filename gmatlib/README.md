# gmatlib - Grant's Matrix Algebra Library

Defines a `Matrix<T>` struct for numeric types and implements several useful functions for matrix algebra purposes.
The source here contains a number of `extern "C"` functions for exposing the types and functions to other languages
as well.

The `Matrix<T>` type is simple, consisting of one contiguous piece of memory to reduce indirection and reduce
the number of allocations needed to construct the matrix.


# Example: Rust
```rust
use gmatlib::Matrix;

// Create a matrix with 3 columns
let a: Matrix<f64> = Matrix::from_vec(3, vec![
  1, 2, 3,
  4, 5, 6,
  7, 8, 9,
]);

// Matrices support appropriate binary operations
let b = a * 3 * vec![0, 1, 0];

// ...and concise indexing 
assert_eq!(
  b[(1, 1)],
  5
);
```
