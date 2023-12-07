use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use std::{format, io::prelude::*, net::TcpStream};
use mrpc::stub::ClientStub;

#[pyclass]
pub struct BaseClient {
    service: String,
    package: String,
    service_id: u32,
    path: String,
    client: Box<ClientStub>,
}

#[pymethods]
impl BaseClient {
    #[new]
    fn new(service: String, package: String, service_id: u32, path: String, addr: String) -> PyResult<Self> {

        match ClientStub::connect(addr) {
            Ok(c) => Ok(BaseClient {
                client: Box::new(c),
                service,
                package,
                service_id,
                path
            }),
            Err(e) => Err(PyBaseException::new_err(format!(
                "Failed to connect: {:?}",
                e
            ))),
        }
    }

    fn initiate_call(&self) -> PyResult<u64> {
        let cid = self.client.initiate_call();
        Ok(cid.0)
    }

    fn unary<T>(&self, service_id: u32, func_id: u32, call_id: u64, req: PyObject) {
        match smol::block_on(self.client.unary(service_id, func_id, call_id, req)) {
            Ok(r) => {},
            Err(e) => {}
        }
    }


    
}

#[pymodule]
fn python_base_client(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<BaseClient>()?;
    Ok(())
}

