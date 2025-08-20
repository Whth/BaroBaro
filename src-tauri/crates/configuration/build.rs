fn main() -> Result<(), Box<dyn std::error::Error>> {
    transmission::compile_proto("config")
}
