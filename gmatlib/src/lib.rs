/// Contains error type definitions for various functions in this crate. 
pub mod error;
/// Contains the source for the traits implemented for and 
/// operators invoving `Matrix<T>`.
mod trait_impls;
/// Contains `extern "C"` function definitions for linking this library
/// against projects in different languages. Not intended for use in 
/// other Rust projects.
mod ffi;

use std::{fmt::Debug, fmt::Display};
use std::mem;
use std::ops::{AddAssign, MulAssign, Neg};
use anyhow::{Error, Result};
use error::*;
use ffi::CompactMatrix;
use num_traits::Num;
pub use trait_impls::*;

/// A helper trait to constrain the type of the elements of a `Matrix<T>`.
pub trait Element<T>: Num + Copy + Debug + Display + AddAssign + MulAssign + Neg<Output = T> {}

impl Element<f32> for f32 {}
impl Element<f64> for f64 {}
impl Element<i8>  for i8  {}
impl Element<i16> for i16 {}
impl Element<i32> for i32 {}
impl Element<i64> for i64 {}

/// An MxN matrix stored as a single contiguous piece of memory.
#[derive(Clone, Debug, PartialEq)]
pub struct Matrix<T>
where T: Element<T>
{
    rows: usize,
    cols: usize,
    vals: Vec<T>,
}

impl <T> Matrix<T>
where T: Element<T>
{
    /// Constructs a new `Matrix<T>` with all indices initialized to `0`.
    /// 
    /// # Example 
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Vec<i32> = Matrix::new(3, 3).into();
    /// assert_eq!(
    ///     a,
    ///     vec![0, 0, 0,
    ///          0, 0, 0,
    ///          0, 0, 0]
    /// );
    /// ```
    pub fn new(rows: usize, cols: usize) -> Matrix<T>
    {
        let mut a = Matrix 
        { 
            rows, 
            cols, 
            vals: Vec::with_capacity(rows * cols),
        };
        
        for _ in 0..a.vals.capacity()
        {
            a.vals.push(T::zero());
        }

        a
    }

    /// Constructs a new identity `Matrix<T>`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Vec<i32> = Matrix::new_identity(3).into();
    /// assert_eq!(
    ///     a, 
    ///     vec![1, 0, 0, 
    ///          0, 1, 0, 
    ///          0, 0, 1]
    /// );
    /// ```
    pub fn new_identity(n: usize) -> Matrix<T>
    {
        let mut a = Matrix::new(n, n);
        
        for i in 0..n
        {
            a[(i, i)] = T::one();
        } 

        a
    }

    /// Constructs a `Matrix<T>` from a `Vec<T>` if it 
    /// has a number of elements evenly divisible by the
    /// number of columns specified.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a = Matrix::from_vec(
    ///     2,
    ///     vec![1, 0,
    ///          0, 1]
    /// ).unwrap();
    /// 
    /// assert_eq!(a, Matrix::new_identity(2));
    /// ```
    pub fn from_vec(cols: usize, vec: Vec<T>) -> Result<Matrix<T>>
    {
        if vec.len() % cols != 0
        {
            return Err(MatrixFromVecError.into())
        }

        Ok(Matrix {
            rows: vec.len() / cols,
            cols,
            vals: vec,
        })
    }

    /// Constructs a single-row `Matrix<T>` from
    /// a given `Vec<T>`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::from_row_vec(
    ///     vec![1, 2, 3]
    /// );
    /// 
    /// assert_eq!(a.get_rows(), 1);
    /// ```
    pub fn from_row_vec(vec: Vec<T>) -> Matrix<T>
    {
        Matrix
        {
            rows: 1,
            cols: vec.len(),
            vals: vec,
        }
    }

    /// Constructs a single-column `Matrix<T>` from
    /// a given `Vec<T>`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::from_col_vec(
    ///     vec![1, 
    ///          2, 
    ///          3]
    /// );
    /// 
    /// assert_eq!(a.get_cols(), 1);
    /// ```
    pub fn from_col_vec(vec: Vec<T>) -> Matrix<T>
    {
        Matrix
        {
            rows: vec.len(),
            cols: 1,
            vals: vec,
        }
    }

    /// Converts this `Matrix<T>` to a `CompactMatrix<T>`,
    /// `mem::forget`ting the memory tied up in `self.vals` 
    /// in the process.
    fn to_compact_matrix(mut self) -> CompactMatrix<T>
    {
        let compact = CompactMatrix
        {
            rows: self.rows,
            cols: self.cols,
            vals: self.vals.as_mut_ptr(),
        };

        // Intentionally leak the `Vec<T>` in `self`.
        // The user will reclaim the EXACT memory leaked
        // here when the reconstruct the Matrix<T> from
        // a CompactMatrix<T> coming in from FFI.
        mem::forget(self);

        compact
    }

    /// Returns the number of rows in the `Matrix<T>`
    /// 
    /// # Example 
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new(4, 5);
    /// 
    /// assert_eq!(4, a.get_rows());
    /// ```
    pub fn get_rows(&self) -> usize
    {
        self.rows
    }

    /// Returns the number of columns in the `Matrix<T>`
    /// 
    /// # Example 
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new(4, 5);
    /// 
    /// assert_eq!(5, a.get_cols());
    /// ```
    pub fn get_cols(&self) -> usize
    {
        self.cols
    }

    /// Swaps the locations of two rows in the matrix.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// a.inplace_row_swap(1, 2);
    /// 
    /// assert_eq!(
    ///     Into::<Vec<i32>>::into(a),
    ///     vec![1, 0, 0, 
    ///          0, 0, 1,
    ///          0, 1, 0]
    /// );
    /// ```
    pub fn inplace_row_swap(&mut self, r1: usize, r2: usize) -> ()
    {
        let mut storage: T;
        for i in 0..self.cols
        {
            storage       = self[(i, r1)];
            self[(i, r1)] = self[(i, r2)];
            self[(i, r2)] = storage;
        } 
    }

    /// Scales the elements in a given row by a given scalar value.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// a.inplace_row_scale(1, 3);
    /// 
    /// assert_eq!(a[(1, 1)], 3);
    /// ```
    pub fn inplace_row_scale(&mut self, row: usize, scalar: T) -> ()
    {
        for i in 0..self.cols
        {
            self[(row, i)] *= scalar;
        }
    }

    /// Scales the elements in the matrix by a given scalar value.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// a.inplace_scale(4);
    /// 
    /// let a_vec: Vec<i32> = a.into();
    /// 
    /// assert_eq!(
    ///     a_vec,
    ///     vec![4, 0, 0,
    ///          0, 4, 0,
    ///          0, 0, 4]
    /// );
    /// ```
    pub fn inplace_scale(&mut self, scalar: T) -> ()
    {
        for i in 0..self.rows
        {
            self.inplace_row_scale(i, scalar);
        }
    }

    /// Adds a row `r2` to another row `r1` in the matrix,
    /// mutating its state.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// a.inplace_row_add(1, 2);
    /// 
    /// assert_eq!(
    ///     Into::<Vec<i32>>::into(a),
    ///     vec![1, 0, 0,
    ///          0, 1, 1,
    ///          0, 0, 1]
    /// );
    /// ```
    pub fn inplace_row_add(&mut self, r1: usize, r2: usize) -> ()
    {
        for i in 0..self.cols
        {
            let addend = self[(r2, i)];
            self[(r1, i)] += addend;
        }
    }

    /// Adds a row `r2` to another row `r1` in the
    /// matrix after scaling row `r2` by a given scalar
    /// value.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// a.inplace_scaled_row_add(1, 2, 3);
    /// 
    /// assert_eq!(
    ///     Into::<Vec<i32>>::into(a),
    ///     vec![1, 0, 0,
    ///          0, 1, 3,
    ///          0, 0, 1]
    /// );
    /// ```
    pub fn inplace_scaled_row_add(&mut self, r1: usize, r2: usize, scalar: T) -> ()
    {
        for i in 0..self.cols
        {
            let addend = self[(r2, i)] * scalar;
            self[(r1, i)] += addend;
        }
    }

    /// Returns the matrix multiplication product of this `Matrix<T>` and 
    /// another `Matrix<T>`. This operation will fail if the operands are
    /// not suitable for [matrix multiplication](https://en.wikipedia.org/wiki/Matrix_multiplication).
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(2);
    /// 
    /// let mut b: Matrix<i32> = Matrix::new(2, 1);
    /// 
    /// b[(0, 0)] = 2;
    /// b[(1, 0)] = 3;
    /// 
    /// let c: Vec<i32> = a.multiply_matrix(&b).unwrap().into();
    /// 
    /// assert_eq!(
    ///     c,
    ///     vec![2, 3]
    /// );
    /// ```
    pub fn multiply_matrix(&self, a: &Matrix<T>) -> Result<Matrix<T>>
    {
        if self.cols != a.rows
        {
            return Err(MatrixMultiplicationError.into())
        }

        let n = self.cols;
        let mut result = Matrix::new(self.rows, a.cols);

        for i in 0..self.rows
        {
            for j in 0..a.cols
            {
                for x in 0..n
                {
                    result[(i, j)] += self[(i, x)] * a[(x, j)]
                }
            }
        }

        Ok(result)
    }

    /// Creates a new `Matrix<T>` with the columns of `a` appended to
    /// the columns of `self`. This operation will fail if the number 
    /// of rows in `a` does not match the number of rows in `self`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// let b: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// let c: Vec<i32> = a.augment_with(&b).unwrap().into();
    /// 
    /// assert_eq!(
    ///     c,
    ///     vec![1, 0, 0, 1, 0, 0,
    ///          0, 1, 0, 0, 1, 0,
    ///          0, 0, 1, 0, 0, 1]
    /// );
    /// ```
    pub fn augment_with(&self, a: &Matrix<T>) -> Result<Matrix<T>>
    {
        if a.get_rows() != self.rows
        {
            return Err(MatrixAugmentationError { a: self.rows, b: a.rows }.into())
        }

        let mut b: Matrix<T> = Matrix::new(self.rows, self.cols + a.cols);

        for i in 0..b.rows
        {
            for j in 0..b.cols
            {
                if j < self.cols
                {
                    b[(i, j)] = self[(i, j)];
                }
                else
                {
                    b[(i, j)] = a[(i, j - self.cols)];
                }
            }
        }

        Ok(b)
    }

    /// Creates a new `Matrix<T>` containing the rows in a range from `r1` to
    /// `r2` and columns in a range from `c1` to `c2`. 
    /// 
    /// # Panics
    /// This operation will panic if the first row or column given is greater 
    /// than or equal to the second row or column given, respectively, or if 
    /// the row or column specified is out of the range of the matrix.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// // Grab the upper right of the identity matrix
    /// let b: Vec<i32> = a.subset(0, 1, 1, 2).into();
    /// 
    /// assert_eq!(
    ///     b,
    ///     vec![0, 0,
    ///          1, 0]
    /// );
    /// ```
    pub fn subset(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> Matrix<T>
    {
        let mut b = Matrix::new(r2 - r1 + 1, c2 - c1 + 1);

        for i in r1..r2+1
        {
            for j in c1..c2+1
            {
                b[(i-r1, j-c1)] = self[(i, j)];
            }
        }

        b
    }

    /// Returns the [trace](https://en.wikipedia.org/wiki/Trace_(linear_algebra)) of a 
    /// `Matrix<T>` if it is square. If not, this method returns a 
    /// `NonSquareMatrixError`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a = Matrix::new_identity(4);
    /// 
    /// let trace: i32 = a.trace().unwrap();
    /// 
    /// assert_eq!(trace, 4);
    /// ```
    pub fn trace(&self) -> Result<T>
    {
        if self.rows != self.cols
        {
            return Err(NonSquareMatrixError.into())
        }

        let mut total: T = T::zero();
        for i in 0..self.rows
        {
            total += self[(i, i)];
        }

        Ok(total)
    }

    /// Transposes this matrix, mirroring it about 
    /// it's diagonal.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a: Matrix<i32> = Matrix::from_vec(
    ///     2, 
    ///     vec![1, 2,
    ///          3, 4,
    ///          5, 6]
    /// ).unwrap();
    /// assert_eq!(a.get_rows(), 3);
    /// assert_eq!(a.get_cols(), 2);
    /// 
    /// // Swap rows and cols
    /// let b = a.transpose();
    /// assert_eq!(b.get_rows(), 2);
    /// assert_eq!(b.get_cols(), 3);
    /// 
    /// let b_vec: Vec<i32> = b.into();
    /// 
    /// assert_eq!(
    ///     b_vec,
    ///     vec![1, 3, 5,
    ///          2, 4, 6]
    /// );
    /// ```
    pub fn transpose(&mut self) -> Matrix<T>
    {
        let mut tspose = self.clone();
        
        let rows = tspose.rows;
        tspose.rows = tspose.cols;
        tspose.cols = rows;

        for i in 0..self.rows
        {    
            for j in 0..self.cols
            {
                tspose[(j, i)] = self[(i, j)];
            }
        }

        tspose
    }

    /// Attempts to invert a 2x2 `Matrix<T>` in-place.
    fn try_inplace_invert_2(&mut self) -> Result<()>
    {
        let a11 = self[(0, 0)];
        let a12 = self[(0, 1)];
        let a21 = self[(1, 0)];
        let a22 = self[(1, 1)];

        let det = a11*a22 - a12*a21;

        if det == T::zero()
        {
            return Err(MatrixInversionError::DeterminantWasZero.into())
        }
        
        self[(0, 0)] =   a22 / det;
        self[(1, 0)] = - a21 / det;
        self[(0, 1)] = - a12 / det;
        self[(1, 1)] =   a11 / det;

        Ok(())
    }

    /// Attempts to invert a 3x3 `Matrix<T>` in-place.
    fn try_inplace_invert_3(&mut self) -> Result<()>
    {
        let a11 = self[(0, 0)];
        let a12 = self[(1, 0)];
        let a13 = self[(2, 0)];
        let a21 = self[(0, 1)];
        let a22 = self[(1, 1)];
        let a23 = self[(2, 1)];
        let a31 = self[(0, 2)];
        let a32 = self[(1, 2)];
        let a33 = self[(2, 2)];

        let det  = a11*a22*a33 + a21*a32*a13 + a31*a12*a23 
                 - a11*a32*a23 - a31*a22*a13 - a21*a12*a33;

        if det == T::zero()
        {
            return Err(MatrixInversionError::DeterminantWasZero.into())
        }

        self[(0, 0)] = (a22 * a33 - a23 * a32) / det;
        self[(1, 0)] = (a23 * a31 - a21 * a33) / det;
        self[(2, 0)] = (a21 * a32 - a22 * a31) / det;
        self[(0, 1)] = (a13 * a32 - a12 * a33) / det;
        self[(1, 1)] = (a11 * a33 - a13 * a31) / det;
        self[(2, 1)] = (a12 * a31 - a11 * a32) / det;
        self[(0, 2)] = (a12 * a23 - a13 * a22) / det;
        self[(1, 2)] = (a13 * a21 - a11 * a23) / det;
        self[(2, 2)] = (a11 * a22 - a12 * a21) / det;

        Ok(())
    }

    /// Attempts to invert a 4x4 `Matrix<T>` in-place.
    fn try_inplace_invert_4(&mut self) -> Result<()>
    {
        let a11 = self[(0, 0)];
        let a12 = self[(1, 0)];
        let a13 = self[(2, 0)];
        let a14 = self[(3, 0)];
        let a21 = self[(0, 1)];
        let a22 = self[(1, 1)];
        let a23 = self[(2, 1)];
        let a24 = self[(3, 1)];
        let a31 = self[(0, 2)];
        let a32 = self[(1, 2)];
        let a33 = self[(2, 2)];
        let a34 = self[(3, 2)];
        let a41 = self[(0, 3)];
        let a42 = self[(1, 3)];
        let a43 = self[(2, 3)];
        let a44 = self[(3, 3)];

        let det  = a11*a22*a33*a44 + a11*a23*a34*a42 + a11*a24*a32*a43 +
                   a12*a21*a34*a43 + a12*a23*a31*a44 + a12*a24*a33*a41 + 
                   a13*a21*a32*a44 + a13*a22*a34*a41 + a13*a24*a31*a42 + 
                   a14*a21*a33*a42 + a14*a22*a34*a43 + a14*a23*a32*a41 -
                   a11*a22*a34*a43 - a11*a23*a32*a44 - a11*a24*a33*a42 -
                   a12*a21*a33*a44 - a12*a23*a34*a41 - a12*a24*a31*a43 -
                   a13*a21*a34*a42 - a13*a22*a31*a44 - a13*a24*a32*a41 -
                   a14*a21*a32*a43 - a14*a22*a33*a41 - a14*a23*a31*a42;

        if det == T::zero()
        {
            return Err(MatrixInversionError::DeterminantWasZero.into());
        }

        self[(0, 0)] = (a22*a33*a44 + a23*a34*a42 + a24*a32*a43 - a22*a34*a43 - a23*a32*a44 - a24*a33*a42) / det;
        self[(1, 0)] = (a12*a34*a43 + a13*a32*a44 + a14*a33*a42 - a12*a33*a44 - a13*a34*a42 - a14*a32*a43) / det;
        self[(2, 0)] = (a12*a23*a44 + a13*a24*a42 + a14*a22*a43 - a12*a24*a43 - a13*a22*a44 - a14*a23*a42) / det;
        self[(3, 0)] = (a12*a24*a33 + a13*a22*a34 + a14*a23*a32 - a12*a23*a34 - a13*a24*a32 - a14*a22*a33) / det;
        self[(0, 1)] = (a21*a34*a43 + a23*a31*a44 + a24*a33*a41 - a21*a33*a44 - a23*a34*a41 - a24*a31*a43) / det;
        self[(1, 1)] = (a11*a33*a44 + a13*a34*a41 + a14*a31*a43 - a11*a34*a43 - a13*a31*a44 - a14*a33*a41) / det;
        self[(2, 1)] = (a11*a24*a43 + a13*a21*a44 + a14*a23*a41 - a11*a23*a44 - a13*a24*a41 - a14*a21*a43) / det;
        self[(3, 1)] = (a11*a23*a34 + a13*a24*a31 + a14*a21*a33 - a11*a24*a33 - a13*a21*a34 - a14*a23*a31) / det;
        self[(0, 2)] = (a21*a32*a44 + a22*a34*a41 + a24*a31*a42 - a21*a34*a42 - a22*a31*a44 - a24*a32*a41) / det;
        self[(1, 2)] = (a11*a34*a42 + a12*a31*a44 + a14*a32*a41 - a11*a32*a44 - a12*a34*a41 - a14*a31*a42) / det;
        self[(2, 2)] = (a11*a22*a44 + a12*a24*a41 + a14*a21*a42 - a11*a24*a42 - a12*a21*a44 - a14*a22*a41) / det;
        self[(3, 2)] = (a11*a24*a32 + a12*a21*a34 + a14*a22*a31 - a11*a22*a34 - a12*a24*a31 - a14*a21*a32) / det;
        self[(0, 3)] = (a21*a33*a42 + a22*a31*a43 + a23*a32*a41 - a21*a32*a43 - a22*a33*a41 - a23*a31*a42) / det;
        self[(1, 3)] = (a11*a32*a43 + a12*a33*a41 + a13*a31*a42 - a11*a33*a42 - a12*a31*a43 - a13*a32*a41) / det;
        self[(2, 3)] = (a11*a23*a42 + a12*a21*a43 + a13*a22*a41 - a11*a22*a43 - a12*a23*a41 - a13*a21*a42) / det;
        self[(3, 3)] = (a11*a22*a33 + a12*a23*a31 + a13*a21*a32 - a11*a23*a32 - a12*a21*a33 - a13*a22*a31) / det;

        Ok(())
    }

    /// Attempts to invert a NxN `Matrix<T>` in-place.
    fn try_inplace_invert_n(&mut self) -> Result<()>
    {
        // assertion that rows == cols has already happened prior to function call
        let n = self.rows;
        let mut inv: Matrix<T> = Matrix::new_identity(n);

        for j in 0..n
        {
            for i in 0..n
            {
                if i == j
                {
                    continue;
                }
                else
                {
                    if self[(j, j)] == T::zero()
                    {
                        return Err(MatrixInversionError::ZeroDuringInversion.into())
                    }
                    let scalar = self[(i, j)] / self[(j, j)];
                    self.inplace_scaled_row_add(i, j, -scalar);
                    inv.inplace_scaled_row_add(i, j, -scalar);
                }
            }
        }

        for i in 0..n
        {
            let scalar: T = T::one() / self[(i, i)];
            self.inplace_row_scale(i, scalar);
            inv.inplace_row_scale(i, scalar);
        }

        *self = inv;
        Ok(())
    }

    /// Attempts to invert a `Matrix<T>` in-place,
    /// returning a `Result<(), Error>` containing any 
    /// relevant failure info if inversion is impossible.
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let mut a = Matrix::new(2, 2);
    /// a[(0, 0)] = -1.0;
    /// a[(0, 1)] =  1.0;
    /// a[(1, 0)] =  1.5;
    /// a[(1, 1)] = -1.0;
    /// 
    /// a.try_inplace_invert().unwrap();
    /// 
    /// let inverse = vec![2.0, 2.0, 
    ///                    3.0, 2.0]; 
    /// 
    /// assert_eq!(Into::<Vec<f64>>::into(a), inverse);
    /// ```
    pub fn try_inplace_invert(&mut self) -> Result<()>
    {
        if self.rows != self.cols
        {
            return Err(NonSquareMatrixError.into())
        }

        if self.rows == 1 && self.vals[0] == T::zero()
        {
            return Err(Error::new(MatrixInversionError::SingularValueWasZero))
        }

        match self.rows {
            1 => self.vals[0] = T::one() / self.vals[0],
            2 => self.try_inplace_invert_2()?,
            3 => self.try_inplace_invert_3()?,
            4 => self.try_inplace_invert_4()?,
            _ => self.try_inplace_invert_n()?,
        };

        Ok(())
    }

}

/// Creates a new row vector `Matrix<T>`
/// 
/// # Example
/// ```
/// use gmatlib::{Matrix, row_vec};
/// 
/// let a: Matrix<i32> = row_vec![1, 2, 3];
/// 
/// assert_eq!(a.get_rows(), 1);
/// assert_eq!(a.get_cols(), 3);
/// ```
#[macro_export]
macro_rules! row_vec {
    ($($e:expr),+ $(,)?) => {
        Matrix::from_row_vec(
            vec![$($e),+]
        )
    };
}

/// Creates a new column vector `Matrix<T>`.
/// 
/// # Example
/// ```
/// use gmatlib::{Matrix, col_vec};
/// 
/// let a: Matrix<i32> = col_vec![1,
///                               2,
///                               3];
/// 
/// assert_eq!(a.get_rows(), 3);
/// assert_eq!(a.get_cols(), 1);
/// ```
#[macro_export]
macro_rules! col_vec {
    ($($e:expr),+ $(,)?) => {
        Matrix::from_col_vec(
            vec![$($e),+]
        )
    };
}
