use pyo3::prelude::*;
use std::format;

#[derive(Copy, Clone, Debug)]
pub enum PayloadType {
    Ping = 0,
    HelloReq = 1,
    HelloResp = 2,
    Error = 4,
}

#[derive(Clone)]
#[pyclass]
pub struct HelloRequest {
    pub message: String,
    pub request_payload: PayloadType,
}


#[pymethods]
impl HelloRequest {
    #[new]
    fn new(message: String) -> Self {
        HelloRequest {
            message,
            request_payload: PayloadType::HelloReq,
        }
    }
}

#[pyclass]
pub struct HelloResponse {
    pub message: String,
    pub request_payload: PayloadType,
}

#[pymethods]
impl HelloResponse {
    #[new]
    fn new(message: String) -> Self {
        HelloResponse {
            message,
            request_payload: PayloadType::HelloResp,
        }
    }

    #[getter]
    fn message(&self) -> PyResult<String> {
        let r = &self.message;
        Ok(r.to_string())
    }

    #[getter]
    fn response_code(&self) -> PyResult<u8> {
        let r = match self.request_payload {
            PayloadType::HelloResp => 2,
            _ => 4,
        };
        Ok(r)
    }
}
