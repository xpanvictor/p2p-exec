use std::io;

fn main() -> Result<(), io::Error> {
    let mut config = prost_build::Config::new();
    config.field_attribute("*", "#[derive(Debug)]");

    prost_build::compile_protos(&["proto/execution.proto"], &["proto"])
}
