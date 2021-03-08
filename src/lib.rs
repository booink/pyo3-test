use pyo3::prelude::*;

#[pyclass]
struct Hello {
    #[pyo3(get)]
    world: String,
}

#[pymodule]
fn pyo3_test(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Hello>()?;

    Ok(())
}
