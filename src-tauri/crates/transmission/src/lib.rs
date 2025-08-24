use std::fs;
use std::path::PathBuf;

const PROTO_DIR: &str = "../../../proto";
const OUT_DIR: &str = "src";

/// Compiles a protobuf file into Rust code with fine grained serde attributes.
pub fn compile_proto_fine_grained(
    name: &str,
    attrs: &Vec<(&str, &str)>,
    field_attrs: &Vec<(&str, &str)>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", PROTO_DIR);
    println!(
        "cargo:rerun-if-changed={}",
        format!("{PROTO_DIR}/{name}.proto")
    );
    fs::create_dir_all(OUT_DIR)?;
    let mut conf = prost_build::Config::new();
    attrs.iter().for_each(|(target, attr)| {
        conf.type_attribute(target, attr);
    });

    field_attrs.iter().for_each(|(target, attr)| {
        conf.field_attribute(target, attr);
    });
    conf.out_dir(OUT_DIR).compile_protos(
        &[format!("{PROTO_DIR}/{name}.proto").as_str()],
        &[PROTO_DIR],
    )?;
    Ok(())
}

/// Compiles a protobuf file into Rust code with a raw attribute.
pub fn compile_proto_raw_with_path(
    path: &str,
    attr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(proto_dir) = PathBuf::from(path).parent() {
        println!("cargo:rerun-if-changed={}", proto_dir.display());
        fs::create_dir_all(OUT_DIR)?;
        prost_build::Config::new()
            .out_dir(OUT_DIR)
            .type_attribute(".", attr)
            .compile_protos(&[path], &[proto_dir])?;
        Ok(())
    } else {
        Err("Invalid proto dir".into())
    }
}

/// Compiles a protobuf file into Rust code.
pub fn compile_proto_raw(name: &str, attr: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw_with_path(format!("{PROTO_DIR}/{name}.proto").as_str(), attr)
}

/// Compiles a protobuf file into Rust code with serde attributes.
pub fn compile_proto(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(name, "#[derive(serde::Serialize, serde::Deserialize)]")
}

/// Compiles a protobuf file into Rust code with serde attributes and extra attributes.
pub fn compile_proto_with(name: &str, extra_attr: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(
        name,
        &format!(
            "#[derive(serde::Serialize, serde::Deserialize)]\n{}",
            extra_attr
        ),
    )
}

/// Compiles a protobuf file into Rust code with serde serialize attributes.
pub fn compile_proto_se(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(name, "#[derive(serde::Serialize)]")
}

/// Compiles a protobuf file into Rust code with serde deserialize attributes.
pub fn compile_proto_de(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(name, "#[derive(serde::Deserialize)]")
}
