use crate::objects::{HelloRequest, HelloResponse};
use crate::rpc_echo;
use crate::rpc_echo::greeter_client::GreeterClient;
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use std::{format, io::prelude::*, net::TcpStream};

#[pyclass]
pub struct Connection {
    client: Box<GreeterClient>,
}

#[pymethods]
impl Connection {
    #[new]
    fn new(addr: String) -> PyResult<Self> {
        match GreeterClient::connect(addr) {
            Ok(c) => Ok(Connection {
                client: Box::new(c),
            }),
            Err(e) => Err(PyBaseException::new_err(format!(
                "Failed to connect: {:?}",
                e
            ))),
        }
    }

    fn say_hello(&mut self, req: HelloRequest) -> PyResult<HelloResponse> {
        let hello_body = rpc_echo::HelloRequest {
            name: req.message.as_bytes().into(),
        };
        let reply = match smol::block_on(self.client.say_hello(hello_body)) {
            Ok(r) => r,
            Err(e) => {
                return Err(PyBaseException::new_err(format!(
                    "Failed to connect: {:?}",
                    e
                )))
            }
        };
        let resp = HelloResponse {
            message: String::from_utf8_lossy(&reply.message).to_string(),
            request_payload: crate::objects::PayloadType::HelloResp,
        };
        Ok(resp)
    }
}
