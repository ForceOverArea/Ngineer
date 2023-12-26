use std::fmt::Display;
use std::ops::{Add, AddAssign, BitOr, Div, Index, IndexMut, Mul, MulAssign, Neg, Sub};
use crate::Matrix;

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
