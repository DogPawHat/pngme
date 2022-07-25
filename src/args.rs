use std::path::PathBuf;

use clap::{Parser, Args, Subcommand};

#[derive(Debug, Parser)]
pub struct PngMeArgs {
    #[clap(subcommand)]
    pub command: PngMeCommands,
}

#[derive(Subcommand, Debug)]
pub enum PngMeCommands {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    #[clap(required = true, parse(from_os_str))]
    pub file_path: PathBuf,
    #[clap(required = true)]
    pub chunk_type: String,
    #[clap(required = true)]
    pub message: String,
    #[clap(required = false, parse(from_os_str))]
    pub output_file: Option<PathBuf>
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    #[clap(required = false, parse(from_os_str))]
    pub file_path: PathBuf,
    #[clap(required = true)]
    pub chunk_type: String,
}


#[derive(Debug, Args)]
pub struct RemoveArgs {
    #[clap(required = false, parse(from_os_str))]
    pub file_path: PathBuf,
    #[clap(required = true)]
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    #[clap(required = false, parse(from_os_str))]
    pub file_path: PathBuf,
}
