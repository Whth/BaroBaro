fn main() -> Result<(), Box<dyn std::error::Error>> {
    prost_build::Config::new()
        .out_dir("src")
        .type_attribute(
            ".",
            "#[derive(serde::Serialize, serde::Deserialize)]",
        )
        .compile_protos(
            &[
                "../../../proto/transmission.proto"
            ],
            &["../../../proto"],
        )?;
    Ok(())
}

