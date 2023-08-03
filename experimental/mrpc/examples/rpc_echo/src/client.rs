pub mod rpc_hello {
    // The string specified here must match the proto package name
    mrpc::include_proto!("rpc_hello");
    // include!("../../../mrpc/src/codegen.rs");
}

use rpc_hello::greeter_client::GreeterClient;
use rpc_hello::HelloRequest;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GreeterClient::connect("0.0.0.0:5000")?;
    let mut time_vec = vec![];
    for i in 0..3000 {
        let req_str = format!("test message number {}", i);
        let start = Instant::now();
        let req = HelloRequest {
            name: req_str.as_bytes().into(),
        };
        smol::block_on(client.say_hello(req))?;
        let duration = start.elapsed();
        time_vec.push(duration.as_millis());
    }
    let f = File::create("mrpc_rust.csv")?;
    let mut buff = BufWriter::new(f);
    for um in time_vec {
        writeln!(buff, "{0}", um)?;
    }
    Ok(())
}
