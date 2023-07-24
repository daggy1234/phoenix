use pyo3::prelude::*;
mod connection;
mod objects;
mod rpc_echo;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rpc_echo_python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_class::<connection::Connection>()?;
    m.add_class::<objects::HelloRequest>()?;
    Ok(())
}
