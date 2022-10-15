use crate::{
    png::Png,
    chunk::Chunk,
    chunk_type::ChunkType,
    args::{EncodeArgs, DecodeArgs, RemoveArgs, PrintArgs},
    Result,
};

use std::str::FromStr;

// Encode the message in the specific PNG file with a type
pub fn encode(args: &EncodeArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    let chunk = Chunk::new(ChunkType::from_str(&args.chunk_type).unwrap(), (&args.message.as_bytes()).to_vec());
    png.append_chunk(chunk);

    if let Some(output_file) = &args.output_file {
        png.write_file(output_file)
    } else {
        png.write_file(&args.file_path)
    }
}

// Decode the message in the specific PNG file according to a chunk type
pub fn decode(args: &DecodeArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;

    if let Some(chunk) = png.chunk_by_type(&args.chunk_type) {
        println!("msg: {}", chunk.data_as_string()?);
        Ok(())
    } else {
        Err(format!(
            "This file does not contain msg of chunk type {}",
            args.chunk_type
        ))?
    }
}

// Remove a message according to chunk type
pub fn remove(args: &RemoveArgs) -> Result<()> {
    let mut png = Png::from_file(&args.file_path)?;
    png.remove_chunk(&args.chunk_type)?;
    png.write_file(&args.file_path)
}

// Print a list of PNG chunks that can be searched for messages
pub fn print_chunks(args: &PrintArgs) -> Result<()> {
    let png = Png::from_file(&args.file_path)?;
    println!(
        "File: {}, Size: {}",
        &args.file_path.display(),
        png.as_bytes().len()
    );

    for (i, chunk) in png.chunks().iter().enumerate() {
        println!(
            "  chunk#{}{{ chunk_type: {}, data_length: {}}}",
            i,
            chunk.chunk_type(),
            chunk.length(),
        );
    }

    Ok(())
}