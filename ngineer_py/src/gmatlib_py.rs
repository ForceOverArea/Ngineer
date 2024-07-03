use pyo3::{create_exception, pyclass, pymethods, PyResult};
use pyo3::exceptions::{PyException, PyIndexError};

create_exception!(ngineer_py, MatrixCreationException,      PyException);
create_exception!(ngineer_py, MatrixInversionException,     PyException);
create_exception!(ngineer_py, MatrixAugmentationException,  PyException);
create_exception!(ngineer_py, MatrixOperationException,     PyException);

#[pyclass]
pub struct Matrix
{
    inner: gmatlib::Matrix<f64>,
}
#[pymethods]
impl Matrix
{
    /// Instantiates a new `Matrix` object
    #[new]
    fn new(mut data: Vec<Vec<f64>>) -> PyResult<Matrix>
    {
        let cols = data[0].len();
        let mut vals = vec![];

        if data.len() == 0
        {
            return Err(MatrixCreationException::new_err(""));
        }

        for list in &mut data
        {
            if list.len() != cols
            {
                return Err(MatrixCreationException::new_err(""));
            }
            vals.append(list)
        }

        Ok(Matrix 
        {
            inner: gmatlib::Matrix::from_vec(cols, vals).unwrap()
        })
    }

    /// Augments a `Matrix` with the right hand operand
    fn __or__(&self, other: &Matrix) -> PyResult<Matrix>
    {
        match self.inner.augment_with(&other.inner)
        {
            Ok(o) => Ok(Matrix { inner: o }),
            Err(e) => Err(MatrixAugmentationException::new_err(e.to_string()))  
        }
    }

    /// Performs element-wise addition between two matrices
    fn __add__(&self, other: &Matrix) -> PyResult<Matrix>
    {
        if self.inner.get_rows() != other.inner.get_rows() ||
           self.inner.get_cols() != other.inner.get_cols()
        {
            return Err(MatrixOperationException::new_err(
                "matrices being added must have the same shape"
            ));
        }

        Ok(Matrix
        {
            inner: &self.inner + &other.inner
        })
    }

    /// Performs element-wise subtraction between two matrices
    fn __sub__(&self, other: &Matrix) -> PyResult<Matrix>
    {
        if self.inner.get_rows() != other.inner.get_rows() ||
           self.inner.get_cols() != other.inner.get_cols()
        {
            return Err(MatrixOperationException::new_err(
                "matrices being subtracted must have the same shape"
            ));
        }

        Ok(Matrix
        {
            inner: &self.inner - &other.inner
        })
    }

    /// Returns the matrix product of the two given matrices
    fn __mul__(&self, other: &Matrix) -> PyResult<Matrix>
    {
        match self.inner.multiply_matrix(&other.inner) 
        {
            Ok(o) => Ok(Matrix { inner: o }), 
            Err(e) => Err(MatrixOperationException::new_err(e.to_string())),
        }
    } 

    /// Formats the `Matrix`'s elements as a string with columns delimited by commas and 
    /// rows delimited by semicolons.
    fn __str__(&self) -> String
    {
        format!("{}", self.inner)
    }

    /// Provides read access to the given element of the `Matrix` 
    fn __getitem__(&self, key: (usize, usize)) -> PyResult<f64>
    {
        if key.0 >= self.inner.get_rows()
        {
            return Err(PyIndexError::new_err(
                "row index out of bounds"
            ));
        }
        else if key.1 >= self.inner.get_cols()
        {
            return Err(PyIndexError::new_err(
                "column index out of bounds"
            ));
        }

        Ok(self.inner[key])
    }

    /// Provides write access to the given element of the `Matrix`
    fn __setitem__(&mut self, key: (usize, usize), newvalue: f64) -> PyResult<()>
    {
        if key.0 >= self.inner.get_rows()
        {
            return Err(PyIndexError::new_err(
                "row index out of bounds"
            ));
        }
        else if key.1 >= self.inner.get_cols()
        {
            return Err(PyIndexError::new_err(
                "column index out of bounds"
            ));
        }

        self.inner[key] = newvalue;
        Ok(())
    }

    /// Inverts the `Matrix`, throwing a `MatrixInversionException` if the inverse does not exist. 
    fn invert(&mut self) -> PyResult<()>
    {
        match self.inner.try_inplace_invert()
        {
            Ok(_)  => Ok(()),
            Err(e) => Err(MatrixInversionException::new_err(e.to_string())),
        }
    }
}