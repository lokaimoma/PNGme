use clap::{Parser, Subcommand};
use std::fs;
use std::io::{Read, Write};
use std::path;

use crate::chunk_type::ChunkType;

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

impl Cli {
    pub fn run() -> crate::Result<()> {
        let cli = Cli::parse();

        match cli.command {
            Command::Encode {
                file_path,
                chunk_type,
                message,
                output_file,
            } => todo!(),
            Command::Decode {
                file_path,
                chunk_type,
            } => todo!(),
            Command::Remove {
                file_path,
                chunk_type,
            } => todo!(),
        }
        todo!()
    }

    fn encode(png_file: path::PathBuf, chunk_type: ChunkType, message: &str) -> crate::Result<()> {
        let mut options = fs::OpenOptions::new();
        let mut png_file = options.read(true).write(true).open(png_file)?;
        let mut png_bytes: Vec<u8> = Vec::new();
        png_file.read_to_end(&mut png_bytes)?;
        let mut png = crate::png::Png::try_from(png_bytes.as_slice())?;
        let chunk =
            crate::chunk::Chunk::new(chunk_type, message.as_bytes().iter().copied().collect());
        png.append_chunk(chunk);
        Ok(png_file.write_all(&png.as_bytes())?)
    }
}
