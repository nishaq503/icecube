use icecube::*;

fn main() -> std::io::Result<()> {
    println!("Hello from `icecube` Rust!");

    let dataset = dataset::DataSet::new(None)?;
    println!("Parsed Dataset: {dataset}");

    Ok(())
}
