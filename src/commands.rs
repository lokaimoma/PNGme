use clap::{Parser, Subcommand};
use std::path;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encode {
        #[arg(short, long)]
        file_path: path::PathBuf,
        #[arg(short, long)]
        chunk_type: String,
        #[arg(short, long)]
        message: String,
        #[arg(short, long)]
        output_file: Option<path::PathBuf>,
    },
    Decode {
        #[arg(short, long)]
        file_path: path::PathBuf,
        #[arg(short, long)]
        chunk_type: String,
    },
    Remove {
        #[arg(short, long)]
        file_path: path::PathBuf,
        #[arg(short, long)]
        chunk_type: String,
    },
}
