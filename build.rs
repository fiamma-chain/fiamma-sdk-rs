use std::{fs, path::PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Build finished");

    let code_string = r#"//@Generated code from proto by tonic, do NOT edit!
pub mod cosmos {
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!("cosmos.base.query.v1beta1.rs");
            }
        }
    }
}

pub mod fiamma {
    pub mod zkpverify {
        include!("fiamma.zkpverify.rs");
    }

    pub mod bitvmstaker {
        include!("fiamma.bitvmstaker.rs");
    }
}

use cosmrs::proto::traits::Name;

macro_rules! impl_name {
    ($type:ty, $package:expr, $name:expr) => {
        impl Name for $type {
            const NAME: &'static str = $name;
            const PACKAGE: &'static str = $package;
        }
    };
}

impl_name!(
    fiamma::zkpverify::MsgSubmitProof,
    "fiamma.zkpverify",
    "MsgSubmitProof"
);

impl_name!(
    fiamma::zkpverify::MsgSubmitCommunityVerification,
    "fiamma.zkpverify",
    "MsgSubmitCommunityVerification"
);

impl_name!(
    fiamma::bitvmstaker::MsgCreateStaker,
    "fiamma.bitvmstaker",
    "MsgCreateStaker"
);

impl_name!(
    fiamma::bitvmstaker::MsgRemoveStaker,
    "fiamma.bitvmstaker",
    "MsgRemoveStaker"
);

impl_name!(
    fiamma::bitvmstaker::MsgRegisterVk,
    "fiamma.bitvmstaker",
    "MsgRegisterVK"
);

impl_name!(
    fiamma::bitvmstaker::MsgRemoveVk,
    "fiamma.bitvmstaker",
    "MsgRemoveVK"
);
"#;

    let out_dir = "src/generated";
    tonic_build::configure().out_dir(out_dir).compile(
        &[
            "src/protos/fiamma/zkpverify/tx.proto",
            "src/protos/fiamma/zkpverify/query.proto",
            "src/protos/fiamma/bitvmstaker/tx.proto",
            "src/protos/fiamma/bitvmstaker/query.proto",
        ],
        &["src/protos", "src/protos/third_party"],
    )?;
    fs::write(PathBuf::from(out_dir).join("mod.rs"), code_string).unwrap();
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
