use std::path::PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: PngMeArgs,
}

#[derive(Debug, Subcommand)]
pub enum PngMeArgs {
    // Encode the message in the specific PNG file with a type
    Encode(EncodeArgs),
    // Decode the message in the specific PNG file according to a chunk type
    Decode(DecodeArgs),
    // Remove a message according to chunk type
    Remove(RemoveArgs),
    // Print a list of PNG chunks that can be searched for messages
    Print(PrintArgs),
}

#[derive(Debug, Args, Clone)]
pub struct EncodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
    pub message: String,
    pub output_file: Option<PathBuf>,
}

#[derive(Debug, Args, Clone)]
pub struct DecodeArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args, Clone)]
pub struct RemoveArgs {
    pub file_path: PathBuf,
    pub chunk_type: String,
}

#[derive(Debug, Args, Clone)]
pub struct PrintArgs {
    pub file_path: PathBuf,
}