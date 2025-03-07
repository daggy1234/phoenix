use std::ffi::OsString;
use std::io;
use std::path::{Path, PathBuf};
use proc_macro2::TokenStream;
use prost_build::Config;
use std::io::{BufRead, BufReader, BufWriter, Write};
use quote::quote;
use syn::Pat;

use crate::attribute::Attributes;
use crate::client;
use crate::server;

/// Simple `.proto` compiling. Use [`configure`] instead if you need more options.
///
/// The include directory will be the parent folder of the specified path.
/// The package name will be the filename without the extension.
pub fn compile_protos(proto: impl AsRef<Path>, out_dir: PathBuf) -> io::Result<()> {
    let proto_path: &Path = proto.as_ref();

    let out_dit_arg = out_dir.clone();
    // directory the main .proto file resides in
    let proto_dir = proto_path
        .parent()
        .expect("proto file should reside in a directory");


    let out_n = proto_path.file_name().and_then(|name| name.to_str()).unwrap().to_string();
    let new_path_rust = str::replace(&out_n, ".proto", ".rs");
    let new_stubs_path_rust = str::replace(&format!("stubs_{}",&out_n), ".proto", ".rs");
    let new_path_python = str::replace(&out_n, ".proto", ".py");

    let mut bulder = self::configure(out_dit_arg);

    bulder.client_attributes.push_struct("HelloRequest", r#"#[pyclass]"#);
    bulder.server_attributes.push_struct(".", r#"#[pyclass]"#);

    bulder.compile(&[proto_path], &[proto_dir])?;

    let mut old_path = out_dir.clone();
    let mut new_path_r = out_dir.clone();
    old_path.push(new_path_rust);
    new_path_r.push((new_stubs_path_rust));
    let to_del_path = old_path.clone();
    let mut new_path_buf = out_dir.clone();
    new_path_buf.push(&new_path_python);
    let file = std::fs::File::open(old_path)?;
    let inp_reader = BufReader::new(file);
    let mut reached_split = false;
    let std_split = "<! BEGIN ACTUAL PYTHON CODE !>".to_string();
    
    let new_out = std::fs::File::create(new_path_buf)?;
    let new_out_r = std::fs::File::create(new_path_r)?;
    

    let mut output_writer = BufWriter::new(new_out);
    let mut output_writer_rust = BufWriter::new(new_out_r);
    for line in inp_reader.lines() {
        let line = line?;
        if (line == std_split) {
            reached_split = true;
        } else if (line == "\n".to_string()) {
            output_writer.write("\n".as_bytes())?;
            output_writer_rust.write("\n".as_bytes())?;
        } else {
            if (reached_split) {
                output_writer.write(line.as_bytes())?;
                output_writer.write("\n".as_bytes())?;

            } else {
                output_writer_rust.write(line.as_bytes())?;
                output_writer_rust.write("\n".as_bytes())?;
            }
        }
    }

    // std::fs::remove_file(to_del_path)?;
    // println!("Old: {:?}\nNew: {:?}", old_path.as_os_str(), new_path_buf.as_os_str());
    // std::fs::rename(old_path, new_path_buf)?;


    Ok(())
}


// Complie protos with default config. Will not be editable rn
pub fn configure(out_dir: PathBuf) -> Builder {
    Builder {
        build_client: true,
        // Do not build server
        build_server: true,
        server_attributes: Attributes::default(),
        client_attributes: Attributes::default(),
        proto_path: "super".to_string(),
        emit_package: true,
        file_descriptor_set_path: None,
        extern_path: Vec::new(),
        field_attributes: Vec::new(),
        type_attributes: Vec::new(),
        compile_well_known_types: false,
        protoc_args: Vec::new(),
        include_file: None,
        out_dir: Some(out_dir),
    }
}

/// Service generator builder.
#[derive(Debug, Clone)]
pub struct Builder {
    // Switches
    pub(crate) build_client: bool,
    pub(crate) build_server: bool,
    // client/server service settings
    pub(crate) server_attributes: Attributes,
    pub(crate) client_attributes: Attributes,
    pub(crate) proto_path: String,
    pub(crate) emit_package: bool,
    // prost settings
    pub(crate) file_descriptor_set_path: Option<PathBuf>,
    pub(crate) extern_path: Vec<(String, String)>,
    pub(crate) field_attributes: Vec<(String, String)>,
    pub(crate) type_attributes: Vec<(String, String)>,
    pub(crate) compile_well_known_types: bool,
    pub(crate) protoc_args: Vec<OsString>,
    pub(crate) include_file: Option<PathBuf>,
    out_dir: Option<PathBuf>,
}

impl Builder {
    /// Compile the .proto files and execute code generation.
    pub fn compile(
        self,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> io::Result<()> {
        // prost_build::compile_protos(protos, includes)
        self.compile_with_config(Config::new(), protos, includes)
    }

    /// Compile the .proto files and execute code generation using a
    /// custom `prost_build::Config`.
    pub fn compile_with_config(
        self,
        mut config: Config,
        protos: &[impl AsRef<Path>],
        includes: &[impl AsRef<Path>],
    ) -> io::Result<()> {
        let out_dir = if let Some(out_dir) = self.out_dir.as_ref() {
            out_dir.clone()
        } else {
            PathBuf::from(std::env::var("OUT_DIR").unwrap())
        };

        config.out_dir(out_dir);
        if let Some(path) = self.file_descriptor_set_path.as_ref() {
            config.file_descriptor_set_path(path);
        }
        for (proto_path, rust_path) in self.extern_path.iter() {
            config.extern_path(proto_path, rust_path);
        }
        for (prost_path, attr) in self.field_attributes.iter() {
            config.field_attribute(prost_path, attr);
        }
        for (prost_path, attr) in self.type_attributes.iter() {
            config.type_attribute(prost_path, attr);
        }
        if self.compile_well_known_types {
            config.compile_well_known_types();
        }
        if let Some(path) = self.include_file.as_ref() {
            config.include_file(path);
        }

        for arg in self.protoc_args.iter() {
            config.protoc_arg(arg);
        }

        config.service_generator(self.service_generator());

        config.compile_protos_mrpc_frontend(protos, includes)?;

        Ok(())
    }

    /// Turn the builder into a `ServiceGenerator` ready to be passed to `prost-build`s
    /// `Config::service_generator`.
    pub fn service_generator(self) -> Box<dyn prost_build::ServiceGenerator> {
        Box::new(ServiceGenerator::new(self))
    }

    /// Enable or disable gRPC client code generation.
    pub fn build_client(mut self, enable: bool) -> Self {
        self.build_client = enable;
        self
    }

    /// Enable or disable gRPC server code generation.
    pub fn build_server(mut self, enable: bool) -> Self {
        self.build_server = enable;
        self
    }

    /// Generate a file containing the encoded `prost_types::FileDescriptorSet` for protocol buffers
    /// modules. This is required for implementing gRPC Server Reflection.
    pub fn file_descriptor_set_path(mut self, path: impl AsRef<Path>) -> Self {
        self.file_descriptor_set_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set the output directory to generate code to.
    ///
    /// Defaults to the `OUT_DIR` environment variable.
    pub fn out_dir(mut self, out_dir: impl AsRef<Path>) -> Self {
        self.out_dir = Some(out_dir.as_ref().to_path_buf());
        self
    }

    /// Declare externally provided Protobuf package or type.
    ///
    /// Passed directly to `prost_build::Config.extern_path`.
    /// Note that both the Protobuf path and the rust package paths should both be fully qualified.
    /// i.e. Protobuf paths should start with "." and rust paths should start with "::"
    pub fn extern_path(mut self, proto_path: impl AsRef<str>, rust_path: impl AsRef<str>) -> Self {
        self.extern_path.push((
            proto_path.as_ref().to_string(),
            rust_path.as_ref().to_string(),
        ));
        self
    }

    /// Add additional attribute to matched messages, enums, and one-offs.
    ///
    /// Passed directly to `prost_build::Config.field_attribute`.
    pub fn field_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, path: P, attribute: A) -> Self {
        self.field_attributes
            .push((path.as_ref().to_string(), attribute.as_ref().to_string()));
        self
    }

    /// Add additional attribute to matched messages, enums, and one-offs.
    ///
    /// Passed directly to `prost_build::Config.type_attribute`.
    pub fn type_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, path: P, attribute: A) -> Self {
        self.type_attributes
            .push((path.as_ref().to_string(), attribute.as_ref().to_string()));
        self
    }

    /// Add additional attribute to matched server `mod`s. Matches on the package name.
    pub fn server_mod_attribute<P: AsRef<str>, A: AsRef<str>>(
        mut self,
        path: P,
        attribute: A,
    ) -> Self {
        self.server_attributes
            .push_mod(path.as_ref().to_string(), attribute.as_ref().to_string());
        self
    }

    /// Add additional attribute to matched service servers. Matches on the service name.
    pub fn server_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, path: P, attribute: A) -> Self {
        self.server_attributes
            .push_struct(path.as_ref().to_string(), attribute.as_ref().to_string());
        self
    }

    /// Add additional attribute to matched client `mod`s. Matches on the package name.
    pub fn client_mod_attribute<P: AsRef<str>, A: AsRef<str>>(
        mut self,
        path: P,
        attribute: A,
    ) -> Self {
        self.client_attributes
            .push_mod(path.as_ref().to_string(), attribute.as_ref().to_string());
        self
    }

    /// Add additional attribute to matched service clients. Matches on the service name.
    pub fn client_attribute<P: AsRef<str>, A: AsRef<str>>(mut self, path: P, attribute: A) -> Self {
        self.client_attributes
            .push_struct(path.as_ref().to_string(), attribute.as_ref().to_string());
        self
    }

    /// Set the path to where mrpc will search for the Request/Response proto structs
    /// live relative to the module where you call `include_proto!`.
    ///
    /// This defaults to `super` since mrpc will generate code in a module.
    pub fn proto_path(mut self, proto_path: impl AsRef<str>) -> Self {
        self.proto_path = proto_path.as_ref().to_string();
        self
    }

    /// Configure Prost `protoc_args` build arguments.
    ///
    /// Note: Enabling `--experimental_allow_proto3_optional` requires protobuf >= 3.12.
    pub fn protoc_arg<A: AsRef<str>>(mut self, arg: A) -> Self {
        self.protoc_args.push(arg.as_ref().into());
        self
    }

    /// Emits GRPC endpoints with no attached package. Effectively ignores protofile package declaration from grpc context.
    ///
    /// This effectively sets prost's exported package to an empty string.
    pub fn disable_package_emission(mut self) -> Self {
        self.emit_package = false;
        self
    }

    /// Enable or disable directing Prost to compile well-known protobuf types instead
    /// of using the already-compiled versions available in the `prost-types` crate.
    ///
    /// This defaults to `false`.
    pub fn compile_well_known_types(mut self, compile_well_known_types: bool) -> Self {
        self.compile_well_known_types = compile_well_known_types;
        self
    }

    /// Configures the optional module filename for easy inclusion of all generated Rust files
    ///
    /// If set, generates a file (inside the `OUT_DIR` or `out_dir()` as appropriate) which contains
    /// a set of `pub mod XXX` statements combining to load all Rust files generated.  This can allow
    /// for a shortcut where multiple related proto files have been compiled together resulting in
    /// a semi-complex set of includes.
    pub fn include_file(mut self, path: impl AsRef<Path>) -> Self {
        self.include_file = Some(path.as_ref().to_path_buf());
        self
    }
}

struct ServiceGenerator {
    builder: Builder,
    clients: String,
    servers: String,
}

impl ServiceGenerator {
    fn new(builder: Builder) -> Self {
        ServiceGenerator {
            builder,
            clients: String::new(),
            servers: String::new(),
        }
    }
}

impl prost_build::ServiceGenerator for ServiceGenerator {
    fn generate(&mut self, service: prost_build::Service, _buf: &mut String) {
        if self.builder.build_server {
            let server = server::generate(
                &service,
                self.builder.emit_package,
                &self.builder.proto_path,
                self.builder.compile_well_known_types,
                &self.builder.server_attributes,
            );
            self.servers = server;
        }

        if self.builder.build_client {
            let client = client::generate(
                &service,
                self.builder.emit_package,
                &self.builder.proto_path,
                self.builder.compile_well_known_types,
                &self.builder.client_attributes,
            );
            self.clients = client;
        }
    }

    fn finalize(&mut self, buf: &mut String) {
        buf.push_str("<! BEGIN ACTUAL PYTHON CODE !>");
        if self.builder.build_client && !self.clients.is_empty() {
            let code = &self.clients;
            buf.push_str(code);

            self.clients = String::new();
        }

        if self.builder.build_server && !self.servers.is_empty() {
            let code = &self.servers;
            buf.push_str(code);
            self.servers = String::new();
        }

        // if self.builder.build_server && !self.servers.is_empty() {
        //     let servers = &self.servers;

        //     let server_service = quote::quote! {
        //         #servers
        //     };

        //     let ast: syn::File = syn::parse2(server_service).expect("not a valid tokenstream");
        //     let code = prettyplease::unparse(&ast);
        //     buf.push_str(&code);

        //     self.servers = TokenStream::default();
        // }
    }

    fn finalize_package(&mut self, package: prost_build::Package, buf: &mut String) {
        let package_mod = quote::format_ident!("proto");
        let mut proto_srcs = Vec::with_capacity(package.source_code_files.len());
        for src_file in package.source_code_files.iter() {
            let src = std::fs::read_to_string(src_file).unwrap();
            proto_srcs.push(src);
        }

        // let tokens = quote! {
        //     pub mod #package_mod {
        //         pub const PROTO_SRCS: &[&str] = &[#(#proto_srcs),*];
        //     }
        // };
        // let ast: syn::File = syn::parse2(tokens).expect("not a valid tokenstream");
        // let code = prettyplease::unparse(&ast);
        // buf.push_str(&code);
    }
}

impl crate::Service for prost_build::Service {
    type Method = prost_build::Method;
    type Comment = String;

    fn name(&self) -> &str {
        &self.name
    }

    fn package(&self) -> &str {
        &self.package
    }

    fn identifier(&self) -> &str {
        &self.proto_name
    }

    fn comment(&self) -> &[Self::Comment] {
        &self.comments.leading[..]
    }

    fn methods(&self) -> &[Self::Method] {
        &self.methods[..]
    }
}

/// Non-path Rust types allowed for request/response types.
const NON_PATH_TYPE_ALLOWLIST: &[&str] = &["()"];

impl crate::Method for prost_build::Method {
    type Comment = String;

    fn name(&self) -> &str {
        &self.name
    }

    fn identifier(&self) -> &str {
        &self.proto_name
    }

    fn comment(&self) -> &[Self::Comment] {
        &self.comments.leading[..]
    }

    fn request_response_name(
        &self,
        proto_path: &str,
        compile_well_known_types: bool,
    ) -> (TokenStream, TokenStream) {
        use quote::ToTokens;

        let convert_type = |proto_type: &str, rust_type: &str| -> TokenStream {
            if (is_google_type(proto_type) && !compile_well_known_types)
                || rust_type.starts_with("::")
                || NON_PATH_TYPE_ALLOWLIST.iter().any(|ty| *ty == rust_type)
            {
                rust_type.parse::<TokenStream>().unwrap()
            } else if rust_type.starts_with("crate::") {
                syn::parse_str::<syn::Path>(rust_type)
                    .unwrap()
                    .to_token_stream()
            } else {
                syn::parse_str::<syn::Path>(&format!("{}", rust_type))
                    .unwrap()
                    .to_token_stream()
            }
        };

        let request = convert_type(&self.input_proto_type, &self.input_type);
        let response = convert_type(&self.output_proto_type, &self.output_type);
        (request, response)
    }

    // TODO: figure out whether we need to specially handle compile_well_known_types
    fn request_response_package(&self, proto_path: &str) -> (Option<String>, Option<String>) {
        let input_package = self.input_package.as_ref().map(|pkg| {
            format!(
                "{}{}{}",
                proto_path,
                if pkg.is_empty() { "" } else { "::" },
                pkg
            )
        });
        let output_package = self.output_package.as_ref().map(|pkg| {
            format!(
                "{}{}{}",
                proto_path,
                if pkg.is_empty() { "" } else { "::" },
                pkg
            )
        });
        (input_package, output_package)
    }
}

fn is_google_type(ty: &str) -> bool {
    ty.starts_with(".google.protobuf")
}
