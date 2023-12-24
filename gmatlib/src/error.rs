use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct MatrixAugmentationError
{
    pub a: usize,
    pub b: usize,
}
impl Display for MatrixAugmentationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(
            f, "failed to augment matrix because they did not have the same number of rows. (A: {}, B: {})", 
            self.a, self.b
        )
    }
}
impl Error for MatrixAugmentationError {}

#[derive(Debug)]
pub struct MatrixSubsetError;
impl Display for MatrixSubsetError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result 
    {
        write!(f, "failed to take subset of matrix because indices were backwards or out of bounds.")
    }
}
impl Error for MatrixSubsetError {}

#[derive(Debug)]
pub enum MatrixInversionError
{
    DeterminantWasZero,
    SingularValueWasZero,
    ZeroDuringInversion
}
impl Display for MatrixInversionError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        match self 
        {
            MatrixInversionError::DeterminantWasZero   => write!(f, "calculated a determinant of 0 while trying to invert matrix."),
            MatrixInversionError::SingularValueWasZero => write!(f, "matrix was zero matrix of size 1 x 1 and cannot be inverted."),
            MatrixInversionError::ZeroDuringInversion  => write!(f, "found a 0 value during the nxn matrix inversion process."),
        }
    }
}
impl Error for MatrixInversionError {}

#[derive(Debug)]
pub struct NonSquareMatrixError;
impl Display for NonSquareMatrixError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "matrix could not be inverted because it did not have the same number of rows and columns.")
    }
}
impl Error for NonSquareMatrixError {}

#[derive(Debug)]
pub struct MatrixFromVecError;
impl Display for MatrixFromVecError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to construct Matrix<T> from Vec<T> because it was not of a len evenly divisible by the number of columns.")
    }
}
impl Error for MatrixFromVecError {}