use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Build finished");

    let code_string = r#"pub mod cosmos {
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
            }
        }
    }
}

pub mod zkpverify;"#;

    let out_dir = "src/generated";
    tonic_build::configure().out_dir(out_dir).compile(
        &["src/protos/zkpverify.proto"],
        &["src/protos", "src/protos/third_party"],
    )?;
    fs::write(PathBuf::from(out_dir).join("mod.rs"), code_string).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
