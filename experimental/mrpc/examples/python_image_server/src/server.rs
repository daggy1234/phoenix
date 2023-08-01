// use crate::image_server::processor_server::{Processor, ProcessorServer};
use crate::image_server::processor_server::{Processor, ProcessorServer};
use crate::image_server::{ImageRequest, ImageResponse};
use crate::objects;
use mrpc;
use mrpc::{RRef, WRef};
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use shm::vec::Vec;
use shmalloc::SharedHeapAllocator;
use std::panic;
use std::process;

#[derive(Debug, Default, Clone)]
#[pyclass(subclass)]
pub struct ImageServer {
    grayscale_wraps: Option<Py<PyAny>>,
    invert_wraps: Option<Py<PyAny>>,
    icon_wraps: Option<Py<PyAny>>,
}

#[mrpc::async_trait]
impl Processor for ImageServer {
    async fn grayscale_image(
        &self,
        req: RRef<ImageRequest>,
    ) -> Result<mrpc::WRef<ImageResponse>, mrpc::Status> {
        let buff = req.image.to_vec();

        let req = objects::ImageRequest { data: buff };
        let obj = self.clone();

        let o = Python::with_gil(|py| -> objects::ImageResponse {
            let args = (req,);
            let kwargs = None;
            let o = obj.grayscale_wraps.unwrap().call(py, args, kwargs);
            let res = match o {
                Ok(r) => r,
                Err(e) => panic!("Error {}", e),
            };
            let o = match res.extract::<objects::ImageResponse>(py) {
                Ok(r) => r,
                Err(e) => panic!("Error {}", e),
            };
            o
        });
        let conv: shm::vec::Vec<u8, shmalloc::SharedHeapAllocator> = o.data.into();
        let resp_b = ImageResponse { image: conv };

        Ok(WRef::new(resp_b))
    }

    async fn invert_image(
        &self,
        req: RRef<ImageRequest>,
    ) -> Result<mrpc::WRef<ImageResponse>, mrpc::Status> {
        let buff = req.image.to_vec();

        let req = objects::ImageRequest { data: buff };
        let obj = self.clone();

        let o = Python::with_gil(|py| -> objects::ImageResponse {
            let args = (req,);
            let kwargs = None;
            let o = obj.invert_wraps.unwrap().call(py, args, kwargs);
            let res = match o {
                Ok(r) => r,
                Err(e) => panic!("Error"),
            };
            let o = match res.extract::<objects::ImageResponse>(py) {
                Ok(r) => r,
                Err(e) => panic!("Error"),
            };
            o
        });
        let conv: shm::vec::Vec<u8, shmalloc::SharedHeapAllocator> = o.data.into();
        let resp_b = ImageResponse { image: conv };

        Ok(WRef::new(resp_b))
    }

    async fn icon_image(
        &self,
        req: RRef<ImageRequest>,
    ) -> Result<mrpc::WRef<ImageResponse>, mrpc::Status> {
        let buff = req.image.to_vec();

        let req = objects::ImageRequest { data: buff };
        let obj = self.clone();

        let o = Python::with_gil(|py| -> objects::ImageResponse {
            let args = (req,);
            let kwargs = None;
            let o = obj.icon_wraps.unwrap().call(py, args, kwargs);
            let res = match o {
                Ok(r) => r,
                Err(e) => panic!("Error"),
            };
            let o = match res.extract::<objects::ImageResponse>(py) {
                Ok(r) => r,
                Err(e) => panic!("Error"),
            };
            o
        });
        let conv: shm::vec::Vec<u8, shmalloc::SharedHeapAllocator> = o.data.into();
        let resp_b = ImageResponse { image: conv };

        Ok(WRef::new(resp_b))
    }
}

#[pymethods]
impl ImageServer {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(ImageServer {
            grayscale_wraps: None,
            icon_wraps: None,
            invert_wraps: None,
        })
    }

    fn add_grayscale_handler(&mut self, wraps: PyObject) -> PyResult<bool> {
        self.grayscale_wraps = Some(wraps);
        Ok(true)
    }

    fn add_invert_handler(&mut self, wraps: PyObject) -> PyResult<bool> {
        self.invert_wraps = Some(wraps);
        Ok(true)
    }
    fn add_icon_handler(&mut self, wraps: PyObject) -> PyResult<bool> {
        self.icon_wraps = Some(wraps);
        Ok(true)
    }

    fn run(self: PyRef<'_, Self>, addr: String) -> PyResult<bool> {
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // invoke the default handler and exit the process
            orig_hook(panic_info);
            process::exit(1);
        }));

        ctrlc::set_handler(move || panic!("CTRL C. STOP SHIT RN")).unwrap();
        let obj = self.clone();
        let out = smol::block_on(async {
            let mut server = match mrpc::stub::LocalServer::bind(addr) {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };
            println!("Starting Server...");
            let myg: ImageServer = obj;

            // Add the Greeter service to the server using the custom MyGreeter implementation.
            let serve = server.add_service(ProcessorServer::new(myg)).serve().await;
            match serve {
                Ok(v) => Ok(true),
                Err(e) => Err(e.to_string()),
            }
        });
        match out {
            Ok(o) => Ok(o),
            Err(e) => Err(PyBaseException::new_err(format!(
                "Failed to connect: {:?}",
                e
            ))),
        }
    }
}
