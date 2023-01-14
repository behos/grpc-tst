use std::{fs::{canonicalize, create_dir_all}, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = canonicalize("../proto")?;
    let proto_def = proto_dir.join("calculator.proto");
    let out_dir = PathBuf::from("src/api");
    if !out_dir.exists() {
        create_dir_all("src/api")?;
    }
    println!("cargo:rerun-if-changed={}", proto_dir.to_string_lossy());
    protoc_grpcio::compile_grpc_protos(
        &[proto_def],
        &[proto_dir],
        &out_dir,
        None
    ).expect("Failed to compile gRPC definitions!");
    Ok(())
}
