use pyo3::{self, pyclass, pymethods, types::PyAnyMethods, Bound};

#[pyclass]
pub struct Matrix
{
    inner: gmatlib::Matrix<f64>,
}
#[pymethods]
impl Matrix
{
    #[new]
    fn new(rows: usize, cols: usize) -> Matrix
    {
        Matrix 
        {
            inner: gmatlib::Matrix::new(rows, cols)
        }
    }

    // #[pyo3(signature = (<self>, other: object) -> object)]
    // fn __mul__<'py>(&self, other: Bound<'py, Matrix>) -> Py<Matrix>
    // {
    //     (Matrix { self.inner * other.inner })
    // }
}