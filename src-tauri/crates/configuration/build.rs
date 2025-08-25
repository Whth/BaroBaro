fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reg = transmission::AttrRegistry::default();

    let (type_attr, field_attr) = reg
        .add_type_global_attr("#[derive(serde::Deserialize, serde::Serialize)]\n#[serde(rename_all = \"camelCase\")]")
        .export();

    transmission::compile_proto_fine_grained("config", &type_attr, &field_attr)
}
