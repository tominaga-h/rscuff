mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rscuff", version, about = "A scaffolding tool written by Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Test,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Test => commands::test::run(),
    }
}
