use crate::cleaner::*;
use crate::filter::*;
use crate::model::BiText;
use pyo3::prelude::*;

mod cleaner;
mod filter;
mod model;
mod moses;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Runs the pipeline
#[pyfunction]
fn run_pipeline(strings: BiText) -> PyResult<BiText> {
    Ok(strings)
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn bitextcleaner(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(run_pipeline, m)?)?;
    m.add_class::<BiText>();
    //m.add_function((BiText::py_new, m)?)?;
    Ok(())
}
