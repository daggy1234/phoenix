// use crate::objects::{HelloRequest, HelloResponse};
use crate::objects::{self, HelloResponse};
use crate::rpc_echo::greeter_server::{Greeter, GreeterServer};
use crate::rpc_echo::{HelloReply, HelloRequest};
use ctrlc;
use mrpc::stub::LocalServer;
use mrpc::{RRef, WRef};
use pyo3::exceptions::PyBaseException;
use pyo3::prelude::*;
use std::panic;
use std::process;
use std::{format, io::prelude::*, net::TcpStream};

#[derive(Debug, Default, Clone, Copy)]
#[pyclass(subclass)]
pub struct MyGreeter {}

#[pymethods]
impl MyGreeter {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(MyGreeter {})
    }

    fn sayhello(&self, req: objects::HelloRequest) -> PyResult<objects::HelloResponse> {
        // unimplemented!("MyGreeter has no sayhello working :(");
        let r = objects::HelloResponse {
            message: req.message,
            request_payload: objects::PayloadType::HelloResp,
        };
        Ok(r)
    }

    fn run(self: PyRef<'_, Self>, addr: String) -> PyResult<bool> {
        let orig_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            // invoke the default handler and exit the process
            orig_hook(panic_info);
            process::exit(1);
        }));

        ctrlc::set_handler(move || panic!("CTRL C. STOP SHIT RN")).unwrap();

        let out = smol::block_on(async {
            let mut server = match mrpc::stub::LocalServer::bind(addr) {
                Ok(s) => s,
                Err(e) => return Err(e.to_string()),
            };
            println!("Starting Server...");
            let myg: MyGreeter = *self;

            // Add the Greeter service to the server using the custom MyGreeter implementation.
            let serve = server.add_service(GreeterServer::new(myg)).serve().await;
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

// Implement the Greeter trait for MyGreeter using async_trait.
#[mrpc::async_trait]
impl Greeter for MyGreeter {
    // Define the say_hello function which takes an RRef<HelloRequest>
    // and returns a Result with a WRef<HelloReply>.
    async fn say_hello(
        &self,
        request: RRef<HelloRequest>,
    ) -> Result<WRef<HelloReply>, mrpc::Status> {
        // Log the received request.
        eprintln!("request: {:?}", request);

        // Create a new HelloReply with a greeting message.
        let message = format!("{}", String::from_utf8_lossy(&request.name));
        let req = objects::HelloRequest {
            message,
            request_payload: objects::PayloadType::HelloReq,
        };

        let o = Python::with_gil(|py| -> HelloResponse {
            let o = self.into_py(py);
            let args = (req,);
            let res = match o.call_method1(py, "sayhello", args) {
                Ok(r) => r,
                Err(e) => panic!("Error"),
            };
            let o = match res.extract::<objects::HelloResponse>(py) {
                Ok(r) => r,
                Err(e) => panic!("Error"),
            };
            o
        });

        let r_msg = o.message;
        let reply = WRef::new(HelloReply {
            message: r_msg.as_bytes().into(),
        });

        Ok(reply)
    }
}

// #[pyclass(extends=MyGreeter, subclass)]
// pub struct Server {
//     addr: String,
// }

// #[pymethods]
// impl Server {
//     #[new]
//     fn new(addr: String) -> (Self, MyGreeter) {
//         let address = &addr;
//         (
//             Server {
//                 addr: address.to_string(),
//             },
//             MyGreeter::new().unwrap(),
//         )
//     }

//     fn sayhello(&self, req: objects::HelloRequest) -> PyResult<objects::HelloResponse> {
//         unimplemented!("Say Hello Nt implemented on server")
//     }

//     fn run(self: PyRef<'_, Self>) -> PyResult<bool> {
//         let orig_hook = panic::take_hook();
//         panic::set_hook(Box::new(move |panic_info| {
//             // invoke the default handler and exit the process
//             orig_hook(panic_info);
//             process::exit(1);
//         }));

//         ctrlc::set_handler(move || panic!("CTRL C. STOP SHIT RN")).unwrap();

//         let out = smol::block_on(async {
//             let mut server = match mrpc::stub::LocalServer::bind(&self.addr) {
//                 Ok(s) => s,
//                 Err(e) => return Err(e.to_string()),
//             };
//             println!("Starting Server...");
//             let supecl = self.into_super();
//             let myg: MyGreeter = *supecl;

//             // Add the Greeter service to the server using the custom MyGreeter implementation.
//             let serve = server.add_service(GreeterServer::new(myg)).serve().await;
//             match serve {
//                 Ok(v) => Ok(true),
//                 Err(e) => Err(e.to_string()),
//             }
//         });
//         match out {
//             Ok(o) => Ok(o),
//             Err(e) => Err(PyBaseException::new_err(format!(
//                 "Failed to connect: {:?}",
//                 e
//             ))),
//         }
//     }
// }
