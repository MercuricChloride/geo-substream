use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("space", "abis/space.json")
        .unwrap()
        .generate()
        .unwrap()
        .write_to_file("src/abi/space.rs")
}
