const PROTO: &str = "../proto/image_server/image_server.proto";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={PROTO}");
    mrpc_build::compile_protos(PROTO)?;
    Ok(())
}
