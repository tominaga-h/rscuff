mod commands;
mod config;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rscuff", version, about = "A scaffolding tool written by Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add(commands::add::AddArgs),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add(args) => commands::add::run(args),
    }
}
