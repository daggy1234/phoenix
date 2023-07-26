///  The request message containing the user's name.
#[repr(C)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub name: ::mrpc::alloc::Vec<u8>,
}
///  The response message containing the greetings
#[repr(C)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HelloReply {
    #[prost(bytes = "vec", tag = "1")]
    pub message: ::mrpc::alloc::Vec<u8>,
}
/// Generate client implementations.
pub mod greeter_client {
    use ::mrpc::stub::{ClientStub, NamedService};
    /// The greeting service definition.
    #[derive(Debug)]
    pub struct GreeterClient {
        stub: ClientStub,
    }
    impl GreeterClient {
        fn update_protos() -> Result<(), ::mrpc::Error> {
            let srcs = [super::proto::PROTO_SRCS].concat();
            ::mrpc::stub::update_protos(srcs.as_slice())
        }
        pub fn connect<A: std::net::ToSocketAddrs>(dst: A) -> Result<Self, ::mrpc::Error> {
            Self::update_protos()?;
            let stub = ClientStub::connect(dst).unwrap();
            Ok(Self { stub })
        }
        /// Sends a greeting
        pub fn say_hello(
            &self,
            req: impl ::mrpc::IntoWRef<super::HelloRequest>,
        ) -> impl std::future::Future<Output = Result<::mrpc::RRef<super::HelloReply>, ::mrpc::Status>>
               + '_ {
            let call_id = self.stub.initiate_call();
            self.stub
                .unary(4059748245u32, 3687134534u32, call_id, req.into_wref())
        }
    }
    impl NamedService for GreeterClient {
        const SERVICE_ID: u32 = 4059748245u32;
        const NAME: &'static str = "rpc_hello.Greeter";
    }
}
/// Generated server implementations.
pub mod greeter_server {
    use ::mrpc::stub::{NamedService, Service};
    #[mrpc::async_trait]
    pub trait Greeter: Send + Sync + 'static {
        /// Sends a greeting
        async fn say_hello(
            &self,
            request: ::mrpc::RRef<super::HelloRequest>,
        ) -> Result<::mrpc::WRef<super::HelloReply>, ::mrpc::Status>;
    }
    /// The greeting service definition.
    #[derive(Debug)]
    pub struct GreeterServer<T: Greeter> {
        inner: T,
    }
    impl<T: Greeter> GreeterServer<T> {
        fn update_protos() -> Result<(), ::mrpc::Error> {
            let srcs = [super::proto::PROTO_SRCS].concat();
            ::mrpc::stub::update_protos(srcs.as_slice())
        }
        pub fn new(inner: T) -> Self {
            Self::update_protos().unwrap();
            Self { inner }
        }
    }
    impl<T: Greeter> NamedService for GreeterServer<T> {
        const SERVICE_ID: u32 = 4059748245u32;
        const NAME: &'static str = "rpc_hello.Greeter";
    }
    #[mrpc::async_trait]
    impl<T: Greeter> Service for GreeterServer<T> {
        async fn call(
            &self,
            req_opaque: ::mrpc::MessageErased,
            read_heap: std::sync::Arc<::mrpc::ReadHeap>,
        ) -> (::mrpc::WRefOpaque, ::mrpc::MessageErased) {
            let func_id = req_opaque.meta.func_id;
            match func_id {
                3687134534u32 => {
                    let req = ::mrpc::RRef::new(&req_opaque, read_heap);
                    let res = self.inner.say_hello(req).await;
                    match res {
                        Ok(reply) => ::mrpc::stub::service_post_handler(reply, &req_opaque),
                        Err(_status) => {
                            todo!();
                        }
                    }
                }
                _ => {
                    todo!("error handling for unknown func_id: {}", func_id);
                }
            }
        }
    }
}
pub mod proto {
    pub const PROTO_SRCS: &[&str] = &[
        "// Copyright 2015 gRPC authors.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage rpc_hello;\n\n// The greeting service definition.\nservice Greeter {\n  // Sends a greeting\n  rpc SayHello (HelloRequest) returns (HelloReply) {}\n}\n\n// The request message containing the user's name.\nmessage HelloRequest {\n  bytes name = 1;\n}\n\n// The response message containing the greetings\nmessage HelloReply {\n  bytes message = 1;\n}\n",
    ];
}
