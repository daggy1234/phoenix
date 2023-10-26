///  The request message containing the user's name.
#[repr(C)]#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(bytes="vec", tag="1")]
    pub name: ::mrpc::alloc::Vec<u8>,
}
///  The response message containing the greetings
#[repr(C)]#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReplied {
    #[prost(bytes="vec", tag="1")]
    pub message: ::mrpc::alloc::Vec<u8>,
}




class GreeterClient(Service):
    
    def __init__(self):
        super().__init__('rpc_hello', 'GreeterClient', 'rpc_hello.Greeter', [])

    def connect(self, addr: str):
        self.connect(addr)

    
    def say_hello(self, req: HelloRequest) -> HelloReplied:
        call_id = self.stub.initiate_call()
        self.stub.unary(4059748245, 3687134534, call_id, req)
        

    