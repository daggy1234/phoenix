use crate::image_server;
use crate::image_server::processor_client::ProcessorClient;
use crate::object;
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;
use shm::vec::Vec;

#[pyclass]
pub struct Connection {
    client: Box<ProcessorClient>,
}

#[pymethods]
impl Connection {
    #[new]
    fn new(addr: String) -> PyResult<Self> {
        match ProcessorClient::connect(addr) {
            Ok(c) => Ok(Connection {
                client: Box::new(c),
            }),
            Err(e) => Err(PyBaseException::new_err(format!(
                "Failed to connect: {:?}",
                e
            ))),
        }
    }

    fn grayscale_image(&mut self, obj: object::ImageRequest) -> PyResult<object::ImageResponse> {
        let dat = obj.data;
        let bdat: Vec<u8, shmalloc::SharedHeapAllocator> = dat.into();
        let imr = image_server::ImageRequest { image: bdat };
        let reply = match smol::block_on(self.client.grayscale_image(imr)) {
            Ok(r) => r,
            Err(e) => {
                return Err(PyBaseException::new_err(format!(
                    "Failed to request: {:?}",
                    e
                )))
            }
        };
        let resp = object::ImageResponse {
            data: reply.image.to_vec(),
        };
        Ok(resp)
    }

    fn invert_image(&mut self, obj: object::ImageRequest) -> PyResult<object::ImageResponse> {
        let dat = obj.data;
        let bdat: Vec<u8, shmalloc::SharedHeapAllocator> = dat.into();
        let imr = image_server::ImageRequest { image: bdat };
        let reply = match smol::block_on(self.client.invert_image(imr)) {
            Ok(r) => r,
            Err(e) => {
                return Err(PyBaseException::new_err(format!(
                    "Failed to request: {:?}",
                    e
                )))
            }
        };
        let resp = object::ImageResponse {
            data: reply.image.to_vec(),
        };
        Ok(resp)
    }

    fn icon_image(&mut self, obj: object::ImageRequest) -> PyResult<object::ImageResponse> {
        let dat = obj.data;
        let bdat: Vec<u8, shmalloc::SharedHeapAllocator> = dat.into();
        let imr = image_server::ImageRequest { image: bdat };
        let reply = match smol::block_on(self.client.icon_image(imr)) {
            Ok(r) => r,
            Err(e) => {
                return Err(PyBaseException::new_err(format!(
                    "Failed to request: {:?}",
                    e
                )))
            }
        };
        let resp = object::ImageResponse {
            data: reply.image.to_vec(),
        };
        Ok(resp)
    }
}
