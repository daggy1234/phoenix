#![feature(arbitrary_self_types)]
use pyo3::prelude::*;
mod image_server;
mod objects;
mod server;

/// A Python module implemented in Rust.
#[pymodule]
fn python_image_server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<server::ImageServer>()?;
    m.add_class::<objects::ImageResponse>()?;
    m.add_class::<objects::ImageRequest>()?;
    Ok(())
}
