use clap::Parser;

use pngme::{
    args::{Cli, PngMeArgs},
    commands::{encode, decode, remove, print_chunks},
    Result,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        PngMeArgs::Encode(args) => encode(args),
        PngMeArgs::Decode(args) => decode(args),
        PngMeArgs::Remove(args) => remove(args),
        PngMeArgs::Print(args) => print_chunks(args),
    }
}
