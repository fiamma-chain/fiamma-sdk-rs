use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Build finished");

    let out_dir = "src/generated";
    tonic_build::configure()
        .out_dir(out_dir)
        .compile(&["src/protos/zkpverify.proto"], &["src/protos", "src/protos/third_party"])?;
    // fs::write(PathBuf::from(out_dir).join("mod.rs"), "pub mod zkpverify;").unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}