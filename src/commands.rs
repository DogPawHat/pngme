use std::fs;
use std::str::FromStr;

use anyhow::{anyhow, Context, Ok};

use crate::Result;
use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

/// Encodes a message into a PNG file and saves the result
pub fn encode(args: EncodeArgs) -> Result<()> {
    let file = fs::read(args.file_path.as_path())?;
    let mut png = Png::try_from(file.as_slice())?;
    let chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().to_vec(),
    );

    png.append_chunk(chunk);

    let output_path = match args.output_file {
        Some(path) => path,
        None => args.file_path,
    };

    fs::write(output_path.as_path(), png.as_bytes()).context("Commands: Could not write to file")
}

/// Searches for a message hidden in a PNG file and prints the message if one is found
pub fn decode(args: DecodeArgs) -> Result<()> {
    let file = fs::read(args.file_path)?;
    let png = Png::try_from(file.as_slice())?;
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| *chunk.chunk_type() == chunk_type);

    match chunk {
        Some(chunk) => {
            let message = chunk.data_as_string()?;
            println!("{}", message);
            Ok(())
        }
        None => Err(anyhow!("No message found")),
    }
}

/// Removes a chunk from a PNG file and saves the result
pub fn remove(args: RemoveArgs) -> Result<()> {
    let file = fs::read(args.file_path.as_path())?;
    let mut png = Png::try_from(file.as_slice())?;

    png.remove_chunk(&args.chunk_type)?;

    fs::write(args.file_path.as_path(), png.as_bytes()).context("Commands: Could not write to file")
}

/// Prints all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> Result<()> {
    let file = fs::read(args.file_path)?;
    let png = Png::try_from(file.as_slice())?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
