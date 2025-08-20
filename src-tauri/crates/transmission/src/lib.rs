use std::fs;

const PROTO_DIR: &str = "../../../proto";
const OUT_DIR: &str = "src";

/// Compiles a protobuf file into Rust code with fine grained serde attributes.
pub fn compile_proto_fine_grained(
    name: &str,
    attrs: &Vec<(&str, &str)>,
    field_attrs: &Vec<(&str, &str)>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", PROTO_DIR);
    fs::create_dir_all(OUT_DIR)?;
    let mut conf = prost_build::Config::new();
    attrs.iter().for_each(|(target, attr)| {
        conf.type_attribute(target, attr);
    });

    field_attrs.iter().for_each(|(target, attr)| {
        conf.field_attribute(target, attr);
    });
    conf.out_dir(OUT_DIR).compile_protos(
        &[format!("../../../proto/{name}.proto").as_str()],
        &[PROTO_DIR],
    )?;
    Ok(())
}
/// Compiles a protobuf file into Rust code.
pub fn compile_proto_raw(name: &str, attr: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed={}", PROTO_DIR);
    fs::create_dir_all(OUT_DIR)?;
    prost_build::Config::new()
        .out_dir(OUT_DIR)
        .type_attribute(".", attr)
        .compile_protos(
            &[format!("../../../proto/{name}.proto").as_str()],
            &[PROTO_DIR],
        )?;
    Ok(())
}

/// Compiles a protobuf file into Rust code with serde attributes.
pub fn compile_proto(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(name, "#[derive(serde::Serialize, serde::Deserialize)]")
}

/// Compiles a protobuf file into Rust code with serde serialize attributes.
pub fn compile_proto_se(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(name, "#[derive(serde::Serialize)]")
}

/// Compiles a protobuf file into Rust code with serde deserialize attributes.
pub fn compile_proto_de(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    compile_proto_raw(name, "#[derive(serde::Deserialize)]")
}
