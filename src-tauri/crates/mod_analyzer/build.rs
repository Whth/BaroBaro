fn main() -> Result<(), Box<dyn std::error::Error>> {
    transmission::compile_proto_fine_grained(
        "mods",
        &vec![
            (
                "mods.FileElement",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            ),
            (
                "mods.FileGroup",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            ),
            (
                "mods.ModList",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            ),
            ("mods.BarotraumaMod", "#[derive(serde::Serialize)]\n#[serde(rename_all = \"camelCase\")]"),
        ],
        &vec![("mods.FileElement.file", "#[serde(rename = \"@file\")]")],
    )
}
