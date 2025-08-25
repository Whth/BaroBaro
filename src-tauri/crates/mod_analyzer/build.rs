fn main() -> Result<(), Box<dyn std::error::Error>> {
    transmission::compile_proto_fine_grained(
        "mods",
        &vec![
            (
                "mods.ModList",
                "#[derive(serde::Serialize, serde::Deserialize)]",
            ),
            ("mods.BarotraumaMod", "#[derive(serde::Serialize, serde::Deserialize)]\n#[serde(rename_all = \"camelCase\", default)]"),
        ],
        &vec![
            ("mods.BarotraumaMod.name", "#[serde(alias = \"@name\")]"),
            ("mods.BarotraumaMod.modVersion", "#[serde(alias = \"@modversion\", alias = \"modVersion\")]"),
            ("mods.BarotraumaMod.corePackage",
             "#[serde(alias = \"@corepackage\", alias = \"corePackage\", deserialize_with=\"crate::deserialize_bool\")]"),
            ("mods.BarotraumaMod.steamWorkshopId",
             "#[serde(alias = \"@steamworkshopid\", alias = \"steamWorkshopId\", deserialize_with=\"crate::deserialize_u64\")]"),
            ("mods.BarotraumaMod.gameVersion", "#[serde(alias = \"@gameversion\", alias = \"gameVersion\")]"),
            ("mods.BarotraumaMod.expectedHash", "#[serde(alias = \"@expectedhash\", alias = \"expectedHash\")]")
        ],
    )
}


