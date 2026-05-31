use color_eyre::eyre::Result;
use colored::Colorize;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

pub fn setup_logging() -> Result<()> {
    let mut config = ConfigBuilder::new();
    config.set_location_level(LevelFilter::Error);
    TermLogger::init(
        LevelFilter::Debug,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )?;

    Ok(())
}

pub fn print_ascii_art(application: &str, version: &str, authors: &[&str]) {
    println!(
        "{}     {}",
        "  ___   __   ____".cyan(),
        "___  __   __ _  ____  ____   __   __    __    ____  ____ ".blue()
    );
    println!(
        "{}   {}",
        " / __) / _\\ (  _ \\".cyan(),
        "/ __)/  \\ (  ( \\(_  _)(  _ \\ /  \\ (  )  (  )  (  __)(  _ \\".blue()
    );
    println!(
        "{}  {}",
        "( (__ /    \\ )   /".cyan(),
        "( (__(  O )/    /  )(   )   /(  O )/ (_/\\/ (_/\\ ) _)  )   /".blue()
    );
    println!(
        "{}   {}",
        " \\___)\\_/\\_/(__\\_)".cyan(),
        "\\___)\\__/ \\_)__) (__) (__\\_) \\__/ \\____/\\____/(____)(__\\_)".blue()
    );
    println!();
    println!(
        "«{}» {} | {} by {}",
        "*".blue(),
        application.blue(),
        format!("v{version}").blue(),
        authors.join(", ").blue()
    );
    println!();
}
