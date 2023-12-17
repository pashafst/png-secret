use std::fs::{read, write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::args::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

pub fn encode_handler(args: EncodeArgs) -> Result<()> {
    let mut png = png_from_path(&args.file_path)?;

    let message: Vec<u8> = args.message.bytes().collect();
    let chunk_type = ChunkType::from_str(&args.chunk_type)?;
    let new_chunk = Chunk::new(chunk_type, message);
    png.append_chunk(new_chunk);
    write_png_file(&args.file_path, png)?;
    println!("Wrote {} for type {}", args.message, chunk_type);
    Ok(())
}

pub fn decode_handler(args: DecodeArgs) -> Result<()> {
    let png = png_from_path(&args.file_path)?;
    let decode_chunk = match png.chunk_by_type(&args.chunk_type) {
        Some(msg) => msg,
        None => panic!("Chunk type {} not found in file.", &args.chunk_type),
    };

    let message = decode_chunk.data_as_string()?;
    println!("{}", message);
    Ok(())
}

pub fn remove_handler(args: RemoveArgs) -> Result<()> {
    let mut png = png_from_path(&args.file_path)?;
    png.remove_chunk(&args.chunk_type)?;
    Ok(write_png_file(&args.file_path, png)?)
}

pub fn print_handler(args: PrintArgs) -> Result<()> {
    let png = png_from_path(&args.file_path)?;
    png.chunks().into_iter().for_each(|c| println!("{}", c));
    Ok(())
}

fn png_from_path(path: &PathBuf) -> Result<Png> {
    let file = read(path)?;
    Png::try_from(file.as_ref())
}

fn write_png_file(path: &PathBuf, png: Png) -> Result<()> {
    let bytes: Vec<u8> = Png::STANDARD_HEADER
        .iter()
        .copied()
        .chain(png.chunks().iter().flat_map(|c| c.as_bytes()))
        .collect();

    Ok(write(path, &bytes)?)
}
