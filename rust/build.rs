use prost_build::Config;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut builder = Config::new();
    builder.protoc_arg("-I=../proto");
    builder.compile_protos(&["apm/v1/action.proto"], &[])?;

    Ok(())
}
