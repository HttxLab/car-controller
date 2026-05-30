use std::{env, path::PathBuf, process::Command};

fn main() -> Result<(), Box<dyn core::error::Error>> {
    generate_planus_code()?;
    Ok(())
}

fn generate_planus_code() -> Result<(), Box<dyn core::error::Error>> {
    println!("cargo:rerun-if-changed=schemas/telemetry.fbs");

    let path = PathBuf::from(env::var("OUT_DIR")?);

    if !Command::new("planus")
        .arg("rust")
        .arg("-o")
        .arg(path.join("telemetry.rs"))
        .arg("schemas/telemetry.fbs")
        .status()?
        .success()
    {
        panic!("Planus failed to compile the Flatbuffers schemas to Rust files.");
    }

    Ok(())
}
