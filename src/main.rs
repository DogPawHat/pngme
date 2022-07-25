mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

use clap::Parser;

pub type Error = anyhow::Error;
pub type Result<T> = anyhow::Result<T>;

use args::{PngMeCommands, PngMeArgs};

fn main() -> Result<()> {
    let args = PngMeArgs::parse();

    match args.command {
        PngMeCommands::Encode(encode_args) => commands::encode(encode_args),
        PngMeCommands::Decode(decode_args) => commands::decode(decode_args),
        PngMeCommands::Remove(remove_args) => commands::remove(remove_args),
        PngMeCommands::Print(print_args) => commands::print_chunks(print_args),
    }
}
