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
