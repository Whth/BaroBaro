fn main() -> Result<(), Box<dyn std::error::Error>> {
    transmission::compile_proto_with("config",
                                     "#[serde(rename_all = \"camelCase\")]")
}
