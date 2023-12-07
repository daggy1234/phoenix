///  The request message containing the user's name.
#[repr(C)]#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(bytes="vec", tag="1")]
    pub name: ::mrpc::alloc::Vec<u8>,
}
///  The response message containing the greetings
#[repr(C)]#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(bytes="vec", tag="1")]
    pub message: ::mrpc::alloc::Vec<u8>,
}
<! BEGIN ACTUAL PYTHON CODE !>


# Service Should be a subclass of ClientStub.
# TODO ask about NamedService?
class GreeterClient(ClientStub):
    
    def __init__(self):
        # Service Name, Package, ServiceId ,Path 
        super().__init__('rpc_hello', 'GreeterClient',4059748245 ,'rpc_hello.Greeter', [])

    def connect(self, addr: str):
        self.connect(addr)

    
    def say_hello(self, req: HelloRequest) -> HelloReply:
        call_id = self.stub.initiate_call()
        self.stub.unary(4059748245, 3687134534, call_id, req)
        

    

class GreeterServer(Service):

    def __init__(self):
        # Service Name, Package, Service_id, Path
        __super__().__init__(Greeter, rpc_hello, 4059748245, rpc_hello.Greeter)
        __super__().add_handler(self.call)

    # Underlying Function that 
    def call(self, context):
        
        if (func_id == 3687134534):
            # Way To Generate Request from Heap
            req = context.request_data
            try:
                res = self.say_hello(req)
            except Exception as e:
                print("Error in Running say_hello")
                raise e
            else:
                # figure out posting reply
                super().post_reply(res, context)

        
        else:
            return Exception("Unknown Method")

    
    def say_hello(self, request: HelloRequest) -> HelloReply:
        raise Exception("Unimplemented method say_hello")
        

    