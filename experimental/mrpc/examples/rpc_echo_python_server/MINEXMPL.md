```rs
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
        // CALL SAYHELLO
        let r = objects::HelloResponse {
            message: req.message,
            request_payload: objects::PayloadType::HelloResp,
        };
        Ok(r)
    }
}

// Implement the Greeter trait for MyGreeter using async_trait.
#[mrpc::async_trait]
impl Greeter for MyGreeter {
    // Define the say_hello function which takes an RRef<HelloRequest>
    // and returns a Result with a WRef<HelloReply>.
    async fn say_hello() {

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

    }
}


```