use pyo3::{pyfunction, PyResult};

#[pyfunction]
fn newton_raphson() -> PyResult<f64>
{
    // geqslib::newton::newton_raphson(f, guess, margin, limit)
    Ok(1.0)
}