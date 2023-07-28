///  The request message containing the user's name.
#[repr(C)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub image: ::mrpc::alloc::Vec<u8>,
}
///  The response message containing the greetings
#[repr(C)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImageResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub image: ::mrpc::alloc::Vec<u8>,
}
/// Generate client implementations.
pub mod processor_client {
    use ::mrpc::stub::{ClientStub, NamedService};
    /// The greeting service definition.
    #[derive(Debug)]
    pub struct ProcessorClient {
        stub: ClientStub,
    }
    impl ProcessorClient {
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
        pub fn grayscale_image(
            &self,
            req: impl ::mrpc::IntoWRef<super::ImageRequest>,
        ) -> impl std::future::Future<
            Output = Result<::mrpc::RRef<super::ImageResponse>, ::mrpc::Status>,
        > + '_ {
            let call_id = self.stub.initiate_call();
            self.stub
                .unary(4144636863u32, 2971192405u32, call_id, req.into_wref())
        }
        pub fn invert_image(
            &self,
            req: impl ::mrpc::IntoWRef<super::ImageRequest>,
        ) -> impl std::future::Future<
            Output = Result<::mrpc::RRef<super::ImageResponse>, ::mrpc::Status>,
        > + '_ {
            let call_id = self.stub.initiate_call();
            self.stub
                .unary(4144636863u32, 2746464688u32, call_id, req.into_wref())
        }
        pub fn icon_image(
            &self,
            req: impl ::mrpc::IntoWRef<super::ImageRequest>,
        ) -> impl std::future::Future<
            Output = Result<::mrpc::RRef<super::ImageResponse>, ::mrpc::Status>,
        > + '_ {
            let call_id = self.stub.initiate_call();
            self.stub
                .unary(4144636863u32, 4016490247u32, call_id, req.into_wref())
        }
    }
    impl NamedService for ProcessorClient {
        const SERVICE_ID: u32 = 4144636863u32;
        const NAME: &'static str = "image_server.Processor";
    }
}
/// Generated server implementations.
pub mod processor_server {
    use ::mrpc::stub::{NamedService, Service};
    #[mrpc::async_trait]
    pub trait Processor: Send + Sync + 'static {
        /// Sends a greeting
        async fn grayscale_image(
            &self,
            request: ::mrpc::RRef<super::ImageRequest>,
        ) -> Result<::mrpc::WRef<super::ImageResponse>, ::mrpc::Status>;
        async fn invert_image(
            &self,
            request: ::mrpc::RRef<super::ImageRequest>,
        ) -> Result<::mrpc::WRef<super::ImageResponse>, ::mrpc::Status>;
        async fn icon_image(
            &self,
            request: ::mrpc::RRef<super::ImageRequest>,
        ) -> Result<::mrpc::WRef<super::ImageResponse>, ::mrpc::Status>;
    }
    /// The greeting service definition.
    #[derive(Debug)]
    pub struct ProcessorServer<T: Processor> {
        inner: T,
    }
    impl<T: Processor> ProcessorServer<T> {
        fn update_protos() -> Result<(), ::mrpc::Error> {
            let srcs = [super::proto::PROTO_SRCS].concat();
            ::mrpc::stub::update_protos(srcs.as_slice())
        }
        pub fn new(inner: T) -> Self {
            Self::update_protos().unwrap();
            Self { inner }
        }
    }
    impl<T: Processor> NamedService for ProcessorServer<T> {
        const SERVICE_ID: u32 = 4144636863u32;
        const NAME: &'static str = "image_server.Processor";
    }
    #[mrpc::async_trait]
    impl<T: Processor> Service for ProcessorServer<T> {
        async fn call(
            &self,
            req_opaque: ::mrpc::MessageErased,
            read_heap: std::sync::Arc<::mrpc::ReadHeap>,
        ) -> (::mrpc::WRefOpaque, ::mrpc::MessageErased) {
            let func_id = req_opaque.meta.func_id;
            match func_id {
                2971192405u32 => {
                    let req = ::mrpc::RRef::new(&req_opaque, read_heap);
                    let res = self.inner.grayscale_image(req).await;
                    match res {
                        Ok(reply) => ::mrpc::stub::service_post_handler(reply, &req_opaque),
                        Err(_status) => {
                            todo!();
                        }
                    }
                }
                2746464688u32 => {
                    let req = ::mrpc::RRef::new(&req_opaque, read_heap);
                    let res = self.inner.invert_image(req).await;
                    match res {
                        Ok(reply) => ::mrpc::stub::service_post_handler(reply, &req_opaque),
                        Err(_status) => {
                            todo!();
                        }
                    }
                }
                4016490247u32 => {
                    let req = ::mrpc::RRef::new(&req_opaque, read_heap);
                    let res = self.inner.icon_image(req).await;
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
        "// Copyright 2015 gRPC authors.\n//\n// Licensed under the Apache License, Version 2.0 (the \"License\");\n// you may not use this file except in compliance with the License.\n// You may obtain a copy of the License at\n//\n//     http://www.apache.org/licenses/LICENSE-2.0\n//\n// Unless required by applicable law or agreed to in writing, software\n// distributed under the License is distributed on an \"AS IS\" BASIS,\n// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.\n// See the License for the specific language governing permissions and\n// limitations under the License.\n\nsyntax = \"proto3\";\n\npackage image_server;\n\n// The greeting service definition.\nservice Processor {\n  // Sends a greeting\n  rpc GrayscaleImage (stream ImageRequest) returns (stream ImageResponse) {}\n  rpc InvertImage (stream ImageRequest) returns (stream ImageResponse) {}\n  rpc IconImage (stream ImageRequest) returns (stream ImageResponse) {}\n}\n\n// The request message containing the user's name.\nmessage ImageRequest {\n  bytes image = 1;\n}\n\n// The response message containing the greetings\nmessage ImageResponse {\n  bytes image = 1;\n}\n",
    ];
}
