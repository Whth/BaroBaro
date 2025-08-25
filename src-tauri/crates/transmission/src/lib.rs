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
    println!("cargo:rerun-if-changed=build.rs");
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


#[derive(Debug, Default)]
pub struct AttrRegistry {
    global_type_attrs: Vec<&'static str>,
    global_field_attrs: Vec<&'static str>,
    field_attrs: Vec<(&'static str, &'static str)>,
    type_attrs: Vec<(&'static str, &'static str)>,
}

impl AttrRegistry {
    pub fn add_type_global_attr(&mut self, attr: &'static str) -> &mut Self {
        self.global_type_attrs.push(attr);
        self
    }
    pub fn add_type_field_attr(&mut self, attr: &'static str) -> &mut Self {
        self.global_field_attrs.push(attr);
        self
    }


    pub fn add_field_attr(&mut self, target: &'static str, attr: &'static str) -> &mut Self {
        self.field_attrs.push((target, attr));
        self
    }

    pub fn add_type_attr(&mut self, target: &'static str, attr: &'static str) -> &mut Self {
        self.type_attrs.push((target, attr));
        self
    }

    pub fn add_global_de(&mut self) -> &mut Self {
        self.add_type_attr(".", "#[derive(serde::Deserialize)]")
    }
    pub fn add_global_se(&mut self) -> &mut Self {
        self.add_type_attr(".", "#[derive(serde::Serialize)]")
    }
    pub fn add_global_se_de(&mut self) -> &mut Self {
        self.add_global_se().add_global_de()
    }

    pub fn export(&self) -> (Vec<(&str, &str)>, Vec<(&str, &str)>) {
        let type_attrs =
            self.global_type_attrs.iter().map(
                |&attr| (".", attr)
            )

                .chain(
                    self.type_attrs.clone()
                )
                .collect();

        let field_attrs =
            self.global_field_attrs.iter().map(
                |&attr| (".", attr)
            )
                .chain(
                    self.field_attrs.clone()
                )
                .collect();

        (type_attrs, field_attrs)
    }
}