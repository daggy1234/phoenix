use pyo3::prelude::*;
mod connection;
mod image_server;
mod object;

/// Formats the sum of two numbers as string.

/// A Python module implemented in Rust.
#[pymodule]
fn python_image_client(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<connection::Connection>()?;
    m.add_class::<object::ImageRequest>()?;
    m.add_class::<object::ImageResponse>()?;
    Ok(())
}
