use base64::{engine::general_purpose, Engine as _};
use pyo3::{prelude::*, types::PyBytes};
use std::format;

#[derive(Clone)]
#[pyclass]
pub struct ImageRequest {
    pub data: Vec<u8>,
}

#[pymethods]
impl ImageRequest {
    #[new]
    fn new(data: Vec<u8>) -> Self {
        ImageRequest { data }
    }

    #[getter]
    fn data(&self) -> PyResult<Vec<u8>> {
        let r = &self.data;
        Ok(r.clone())
        // let o = general_purpose::STANDARD.encode(r);
        // Ok(o)
    }
}

#[derive(Clone)]
#[pyclass]
pub struct ImageResponse {
    pub data: Vec<u8>,
}

#[pymethods]
impl ImageResponse {
    #[new]
    fn new(data: Vec<u8>) -> Self {
        ImageResponse { data }
    }

    #[getter]
    fn data(&self) -> PyResult<Vec<u8>> {
        let r = &self.data;
        // let o = general_purpose::STANDARD.encode(r);
        // Ok(o)
        // let r = &self.data;
        // let o = general_purpose::STANDARD.encode(r);
        Ok(r.clone())
    }
}
