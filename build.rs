use std::{
    env,
    fs::{self, File},
    io::Write as _,
    path::PathBuf,
    process::Command,
};

use toml::Value;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    generate_planus_code()?;
    generate_version_code()?;

    Ok(())
}

fn generate_version_code() -> Result<(), Box<dyn core::error::Error>> {
    let path = PathBuf::from(env::var("OUT_DIR")?).join("version.rs");
    let mut file = File::create(&path)?;

    let commit = env::var("CURRENT_COMMIT").unwrap_or_else(|_| "unknown".to_string());
    let build = env::var("CURRENT_BUILD").unwrap_or_else(|_| "0".to_string());

    let version = extract_version()?;

    writeln!(file, "pub const VERSION: Version = Version {{")?;
    writeln!(file, "    major: {},", version.0)?;
    writeln!(file, "    minor: {},", version.1)?;
    writeln!(file, "    patch: {},", version.2)?;
    writeln!(file, "    build: {build},")?;
    writeln!(file, "    commit: \"{commit}\",").unwrap();
    writeln!(file, "    stage: Stage::{},", version.3).unwrap();
    writeln!(file, "}};").unwrap();

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

fn extract_version() -> Result<(u16, u16, u16, String), Box<dyn core::error::Error>> {
    let toml = toml::from_str::<Value>(&fs::read_to_string("Cargo.toml")?)?;

    let version = toml["package"]["version"]
        .as_str()
        .ok_or("Unable to get version from Cargo.toml")?
        .split('-')
        .collect::<Vec<_>>();
    let numbers = version
        .first()
        .ok_or("The version found in the Cargo.toml file is not formatted correctly.")?
        .split(".")
        .map(|number| number.parse::<u16>())
        .collect::<Result<Vec<_>, _>>()?;

    if numbers.len() != 3 {
        return Err("The version found in the Cargo.toml is not formatted correctly.".into());
    }

    let stage = if version.len() > 1 {
        version[1][0..1].to_uppercase() + &version[1][1..]
    } else {
        "Stable".to_owned()
    };
    Ok((numbers[0], numbers[1], numbers[2], stage))
}
