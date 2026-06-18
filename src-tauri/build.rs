fn main() -> Result<(), Box<dyn std::error::Error>> {
    transmission::compile_proto_raw_with_path(
        "../proto/build_info.proto",
        "#[derive(serde::Serialize, serde::Deserialize)]",
    )?;
    built::write_built_file()?;
    tauri_build::build();
    Ok(())
}
