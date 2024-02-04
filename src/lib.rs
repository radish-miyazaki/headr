use std::error::Error;
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(name = "headr", version = "0.1.0", author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>", about = "Rust head")]
pub struct Args {
    #[arg(
        value_name = "FILE",
        help = "Input file(s)",
        default_value = "-"
    )]
    files: Vec<String>,
    #[arg(
        value_name = "LINES",
        help = "Number of lines",
        short = 'n',
        long = "lines",
        default_value = "10",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64,
    #[arg(
        value_name = "BYTES",
        help = "Number of bytes",
        short = 'c',
        long = "bytes",
        conflicts_with = "lines",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    dbg!(args);

    Ok(())
}
