use std::{fs::{canonicalize, create_dir_all}, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let proto_dir = canonicalize("../proto")?;
    let proto_def = proto_dir.join("calculator.proto");
    let out_dir = PathBuf::from("src/api");
    if !out_dir.exists() {
        create_dir_all("src/api")?;
    }
    tonic_build::configure()
        .build_client(false)
        .out_dir("src/api")
        .compile(
            &[proto_def],
            &[proto_dir],
        )?;
    Ok(())
}
