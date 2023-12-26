/// Contains error type definitions for various functions in this crate. 
pub mod error;
/// Contains `extern "C"` function definitions for linking this library
/// against projects in different languages. Not intended for use in 
/// other Rust projects.
mod ffi;

use std::fmt::{Debug, Display};
use std::mem;
use std::ops::{Add, AddAssign, BitOr, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};
use anyhow::{Error, Result};
use error::*;
use ffi::CompactMatrix;

/// An MxN matrix stored as a single contiguous piece of memory.
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Matrix<T>
{
    rows: usize,
    cols: usize,
    vals: Vec<T>,
}

impl <T> Matrix<T>
where
    T: Add + AddAssign + Add<Output = T>
     + Copy
     + Div + Div<Output = T>
     + From<i32>
     + Mul + MulAssign + Mul<Output = T>
     + Neg + Neg<Output = T>
     + PartialEq
     + Sub + Sub<Output = T>
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
            a.vals.push(0.into());
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
            a[(i, i)] = 1.into();
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
    /// 
    /// a.inplace_transpose();
    /// 
    /// let a_vec: Vec<i32> = a.into();
    /// 
    /// assert_eq!(
    ///     a_vec,
    ///     vec![1, 3, 5,
    ///          2, 4, 6]
    /// );
    /// ```
    pub fn inplace_transpose(&mut self) -> ()
    {
        let mut storage: T;
        let mut ci: usize; // current index
        let mut ti: usize; // transposed index to move value to

        for i in 0..self.rows
        {
            for j in 0..self.cols
            {
                if i == j // ignore diagonal. This is not affected by transposing a matrix
                {
                    continue;
                }
                ci = self.cols * i + j;
                ti = self.rows * j + i;

                storage       = self.vals[ti];
                self.vals[ti] = self.vals[ci];
                self.vals[ci] = storage;
            }
        }

        // switch the number of rows and columns
        let storage = self.rows;
        self.rows = self.cols;
        self.cols = storage;
    }

    /// Attempts to invert a 2x2 `Matrix<T>` in-place.
    fn try_inplace_invert_2(&mut self) -> Result<()>
    {
        let a11 = self[(0, 0)];
        let a12 = self[(0, 1)];
        let a21 = self[(1, 0)];
        let a22 = self[(1, 1)];

        let det = a11*a22 - a12*a21;

        if det == 0.into()
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

        if det == 0.into()
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

        if det == 0.into()
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
                    if self[(j, j)] == 0.into()
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
            let scalar: T = <i32 as Into<T>>::into(1) / self[(i, i)];
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

        if self.rows == 1 && self.vals[0] == 0.into()
        {
            return Err(Error::new(MatrixInversionError::SingularValueWasZero))
        }

        match self.rows {
            1 => self.vals[0] = <i32 as Into<T>>::into(1) / self.vals[0],
            2 => self.try_inplace_invert_2()?,
            3 => self.try_inplace_invert_3()?,
            4 => self.try_inplace_invert_4()?,
            _ => self.try_inplace_invert_n()?,
        };

        Ok(())
    }

}

impl <T> Display for Matrix<T>
where T:
    Display
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        let mut output = String::new();
        let r = self.rows - 1;
        let c = self.cols - 1;

        for i in 0..r
        {
            for j in 0..c
            {
                output += format!("{}, ", self[(i, j)]).as_str();
            }
            output += format!("{}; ", self[(i, c)]).as_str();
        }

        for j in 0..c
        {
            output += format!("{}, ", self[(r, j)]).as_str();
        }
        output += format!("{}]", self[(r, c)]).as_str();

        write!(f, "{}", output)
    }
}

impl <T> BitOr for Matrix<T>
where
    T: Add + AddAssign + Add<Output = T>
     + Copy
     + Div + Div<Output = T>
     + From<i32>
     + Mul + MulAssign + Mul<Output = T>
     + Neg + Neg<Output = T>
     + PartialEq
     + Sub + Sub<Output = T>
{
    type Output = Matrix<T>;

    /// Shorthand for `Matrix::augment_with`.
    /// 
    /// # Panics
    /// This operation will panic if the number of
    /// rows in both operands do not match.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// let b: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// let c: Vec<i32> = (a | b).into();
    /// 
    /// assert_eq!(
    ///     c, 
    ///     vec![1, 0, 0, 1, 0, 0,
    ///          0, 1, 0, 0, 1, 0,
    ///          0, 0, 1, 0, 0, 1]
    /// );
    /// ``` 
    fn bitor(self, rhs: Self) -> Matrix<T> 
    {
        self.augment_with(&rhs).unwrap()
    }
}

impl <T> BitOr for &Matrix<T>
where
    T: Add + AddAssign + Add<Output = T>
     + Copy
     + Div + Div<Output = T>
     + From<i32>
     + Mul + MulAssign + Mul<Output = T>
     + Neg + Neg<Output = T>
     + PartialEq
     + Sub + Sub<Output = T>
{
    type Output = Matrix<T>;

    /// Shorthand for `Matrix::augment_with`.
    /// 
    /// # Panics
    /// This operation will panic if the number of
    /// rows in both operands do not match.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// let b: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// let c: Vec<i32> = (a | b).into();
    /// 
    /// assert_eq!(
    ///     c, 
    ///     vec![1, 0, 0, 1, 0, 0,
    ///          0, 1, 0, 0, 1, 0,
    ///          0, 0, 1, 0, 0, 1]
    /// );
    /// ``` 
    fn bitor(self, rhs: Self) -> Self::Output {
        self.augment_with(&rhs).unwrap()
    }
}

impl <T> Mul for Matrix<T>
where
    T: Add + AddAssign + Add<Output = T>
    + Copy
    + Div + Div<Output = T>
    + From<i32>
    + Mul + MulAssign + Mul<Output = T>
    + Neg + Neg<Output = T>
    + PartialEq
    + Sub + Sub<Output = T>
{
    type Output = Self;

    /// Multiplies a matrix by another scalar: `T`, `Vec<T>`, 
    /// or `Matrix<T>`. For matrix-scalar multiplication, 
    /// this scales the elements in the left operand. For 
    /// matrix-vector multiplication, this operator treats
    /// the left-hand operand as a row vector and the right-hand
    /// operand as a column vector. For pure matrix multiplication,
    /// this returns the [matrix product](https://en.wikipedia.org/wiki/Matrix_multiplication)
    /// of the operands. 
    /// 
    /// # Panics
    /// This operation will panic if the operands 
    /// are not suitable for multiplication (i.e.
    /// matrices/vectors are not the correct shape.)
    /// 
    /// # Example
    /// Matrix-matrix multiplication:
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// let b: Vec<i32> = vec![1, 2, 3];
    /// 
    /// let c: Vec<i32> = (a * b).into(); // `b` is a COLUMN vector here. It is the right-hand operand.
    /// assert_eq!(
    ///     c,
    ///     vec![1,
    ///          2,
    ///          3]
    /// );
    /// ```
    fn mul(self, rhs: Self) -> Self::Output 
    {
        self.multiply_matrix(&rhs).unwrap()
    }
}

impl <T> Mul<Vec<T>> for Matrix<T>
where
    T: Add + AddAssign + Add<Output = T>
    + Copy
    + Div + Div<Output = T>
    + From<i32>
    + Mul + MulAssign + Mul<Output = T>
    + Neg + Neg<Output = T>
    + PartialEq
    + Sub + Sub<Output = T>
{
    type Output = Matrix<T>;

    /// Multiplies a matrix by another scalar: `T`, `Vec<T>`, 
    /// or `Matrix<T>`. For matrix-scalar multiplication, 
    /// this scales the elements in the left operand. For 
    /// matrix-vector multiplication, this operator treats
    /// the left-hand operand as a row vector and the right-hand
    /// operand as a column vector. For pure matrix multiplication,
    /// this returns the [matrix product](https://en.wikipedia.org/wiki/Matrix_multiplication)
    /// of the operands. 
    /// 
    /// # Panics
    /// This operation will panic if the operands 
    /// are not suitable for multiplication (i.e.
    /// matrices/vectors are not the correct shape.)
    /// 
    /// # Example
    /// Matrix-matrix multiplication:
    /// ```
    /// ```
    fn mul(self, rhs: Vec<T>) -> Self::Output {
        self * Matrix { rows: rhs.len(), cols: 1, vals: rhs }
    }
}

impl <T> Mul<Matrix<T>> for Vec<T>
where
    T: Add + AddAssign + Add<Output = T>
    + Copy
    + Div + Div<Output = T>
    + From<i32>
    + Mul + MulAssign + Mul<Output = T>
    + Neg + Neg<Output = T>
    + PartialEq
    + Sub + Sub<Output = T>
{
    type Output = Matrix<T>;

    /// Multiplies a matrix by another scalar: `T`, `Vec<T>`, 
    /// or `Matrix<T>`. For matrix-scalar multiplication, 
    /// this scales the elements in the left operand. For 
    /// matrix-vector multiplication, this operator treats
    /// the left-hand operand as a row vector and the right-hand
    /// operand as a column vector. For pure matrix multiplication,
    /// this returns the [matrix product](https://en.wikipedia.org/wiki/Matrix_multiplication)
    /// of the operands. 
    /// 
    /// # Panics
    /// This operation will panic if the operands 
    /// are not suitable for multiplication (i.e.
    /// matrices/vectors are not the correct shape.)
    /// 
    /// # Example
    /// Matrix-vector multiplication:
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// let b: Vec<i32> = vec![1, 2, 3];
    /// 
    /// let c: Vec<i32> = (a * b).into(); // `b` is a COLUMN vector here. It is the right-hand operand.
    /// assert_eq!(
    ///     c,
    ///     vec![1,
    ///          2,
    ///          3]
    /// );
    /// ```
    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        Matrix { rows: 1, cols: self.len(), vals: self } * rhs
    }
}

impl <T> Into<Vec<T>> for Matrix<T>
{
    fn into(self) -> Vec<T> 
    {
        self.vals
    }
}

impl <T> Index<(usize, usize)> for Matrix<T>
{
    type Output = T;

    /// Allows for getting specific elements from 
    /// a `Matrix<T>`. This automatically calculates the
    /// 1-D `Vec` index needed to retrieve the desired 
    /// value from the user-facing 2-D `Matrix`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// // gmatlib Matrix:
    /// let a: Matrix<i32> = Matrix::new_identity(3);
    /// 
    /// // Contiguous Vec:
    /// let b = vec![1, 0, 0,
    ///              0, 1, 0,
    ///              0, 0, 1];
    /// 
    /// // both have the same actual structure
    /// // but indexing the matrix communicates
    /// // what we're doing better:
    /// assert_eq!(a[(1, 1)], b[4]);
    /// ```
    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &T 
    {
        if index.0 >= self.rows || index.1 >= self.cols 
        {
            panic!("index out of bounds: the matrix has {} rows and {} cols but the index was [({}, {})]", self.rows, self.cols, index.0, index.1)
        }
        &(self.vals[index.0 * self.cols + index.1])
    }
}

impl <T> IndexMut<(usize, usize)> for Matrix<T>
{
    /// Allows for getting specific elements from 
    /// a `Matrix<T>`. This automatically calculates the
    /// 1-D `Vec` index needed to retrieve the desired 
    /// value from the user-facing 2-D `Matrix`.
    /// 
    /// # Example
    /// ```
    /// use gmatlib::Matrix;
    /// 
    /// // gmatlib Matrix:
    /// let mut a: Matrix<i32> = Matrix::new_identity(3);
    /// a[(1, 1)] = 2;
    /// 
    /// // Contiguous Vec:
    /// let b = vec![1, 0, 0,
    ///              0, 2, 0,
    ///              0, 0, 1];
    /// 
    /// // both have the same actual structure
    /// // but indexing the matrix communicates
    /// // what we're doing better:
    /// assert_eq!(a[(1, 1)], b[4]);
    /// ```
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T 
    {
        &mut (self.vals[index.0 * self.cols + index.1])
    }
}

#[test]
fn ensure_try_inplace_invert_3_works_as_expected()
{
    let mut a = Matrix::from_vec(
        3, 
        vec![ 1.0,  2.0, -1.0,
              2.0,  1.0,  2.0,
             -1.0,  2.0,  1.0]
    ).unwrap();

    a.try_inplace_invert().unwrap();

    let a_vec: Vec<f64> = <Matrix<f64> as Into<Vec<f64>>>::into(a);

    let check = vec![
        3.0/16.0 as f64, 1.0/4.0 as f64, -5.0/16.0 as f64,
        1.0/4.0  as f64,     0.0 as f64,  1.0/4.0  as f64,
       -5.0/16.0 as f64, 1.0/4.0 as f64,  3.0/16.0 as f64
    ];

    assert_eq!(a_vec, check);
}

#[test]
fn ensure_try_inplace_invert_4_works_as_expected()
{
    let mut a = Matrix::from_vec(
        4, 
        vec![ 4.0,  0.0,  0.0,  0.0,
              0.0,  0.0,  2.0,  0.0,
              0.0,  1.0,  2.0,  0.0,
              1.0,  0.0,  0.0,  1.0]
    ).unwrap();

    a.try_inplace_invert().unwrap();

    let a_vec: Vec<f64> = <Matrix<f64> as Into<Vec<f64>>>::into(a);

    let check = vec![
        0.25, 0.0, 0.0, 0.0,
        0.0, -1.0, 1.0, 0.0,
        0.0,  0.5, 0.0, 0.0,
       -0.25, 0.0, 0.0, 1.0
    ];

    assert_eq!(a_vec, check);
}

#[test]
fn ensure_try_inplace_invert_n_works_as_expected()
{
    let mut a = Matrix::from_vec(
        5,
        vec![ 3.0, 11.0,  2.0, 17.0, 22.0, 
              4.0, 10.0, 12.0, 18.0, 23.0, 
              8.0,  9.0, 14.0, 19.0, 24.0, 
              5.0,  7.0, 15.0, 20.0, 25.0, 
              6.0, 13.0, 16.0, 21.0, 26.0]
    ).unwrap();

    a.try_inplace_invert().unwrap();

    let shift = 10.0f64.powi(4);

    let a_vec: Vec<f64> = <Matrix<f64> as Into<Vec<f64>>>::into(a)
        .into_iter()
        .map(|i: f64| {
            (i * shift).floor() / shift
        })
        .collect();

    let check = vec![
        0.0181, -0.1819,  0.2909, -0.1091, -0.0182,
       -0.0091,  0.0909, -0.0205, -0.1955,  0.1340,
       -0.1182,  0.1818, -0.0160, -0.0410, -0.0069,
        0.4236, -2.2364, -0.5469,  0.9081,  1.2513,
       -0.2691,  1.6909,  0.3945, -0.5855, -1.0310
    ];

    assert_eq!(a_vec, check);
}
