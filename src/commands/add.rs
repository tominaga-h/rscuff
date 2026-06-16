use crate::config;
use camino::Utf8PathBuf;
use clap::Args;

#[derive(Args)]
pub struct AddArgs {
    /// Target file path
    pub file: String,

    /// Group name
    #[arg(short, long)]
    pub group: Option<String>,
}

pub fn run(args: AddArgs) {
    let arg_file = args.file;
    if arg_file.is_empty() {
        eprintln!("Error: file cannot be empty");
        std::process::exit(1);
    }

    let arg_file_path = Utf8PathBuf::from(arg_file);
    if !arg_file_path.exists() {
        eprintln!("Error: file does not exist: '{}'", arg_file_path);
        std::process::exit(1);
    }

    let storage = config::get_storage();
    if !storage.exists() {
        storage.create().expect("failed to create storage");
    }

    // let group = args.group.unwrap_or("default".to_string());
}
