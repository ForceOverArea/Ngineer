mod gmatlib_py;
mod geqslib_py;

use pyo3::{pymodule, Bound, PyResult};
use pyo3::types::PyModule;
use gmatlib_py::Matrix;

#[pymodule]
fn ngineer_py(m: &Bound<'_, PyModule>) -> PyResult<()>
{
    m.add_class::<Matrix>()?;
    Ok(())
}