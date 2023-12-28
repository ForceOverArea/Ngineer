# gmatlib - Grant's Matrix Algebra Library

Defines a `Matrix<T>` struct for numeric types and implements several useful functions for matrix algebra purposes.
The source here contains a number of `extern "C"` functions for exposing the types and functions to other languages
as well.

The `Matrix<T>` type is simple, consisting of one contiguous piece of memory to reduce indirection and reduce
the number of allocations needed to construct the matrix.


# Example
```rust
use gmatlib::Matrix;

// Create a matrix with 3 columns
let a: Matrix<f64> = Matrix::from_vec(3, vec![
    1.0, 2.0, 3.0,
    4.0, 5.0, 6.0,
    7.0, 8.0, 9.0,
]).unwrap();

// Matrices support appropriate binary operations
let b = vec![0.0, 1.0, 0.0] * &(&a * 3.0);

// ...and concise indexing 
assert_eq!(
    b[(0, 1)],
    5.0
);
```
