use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::{Parser};

#[derive(Parser, Debug)]
#[command(
    name = "headr",
    version = "0.1.0",
    author = "Radish-Miyazaki <y.hidaka.kobe@gmail.com>",
    about = "Rust head",
)]
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
        value_parser = parse_lines_positive_int,
        default_value = "10",
    )]
    lines: usize,

    #[arg(
        value_name = "BYTES",
        help = "Number of bytes",
        short = 'c',
        long = "bytes",
        value_parser = parse_bytes_positive_int,
        conflicts_with = "lines",
    )]
    bytes: Option<usize>,
}

fn parse_lines_positive_int(s: &str) -> Result<usize, String> {
    parse_positive_int(s, "line")
}

fn parse_bytes_positive_int(s: &str) -> Result<usize, String> {
    parse_positive_int(s, "byte")
}

fn parse_positive_int(s: &str, typ: &str) -> Result<usize, String> {
    match s.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(format!("illegal {} count -- {}", typ, s))
    }
}

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

pub fn get_args() -> MyResult<Args> {
    let args = Args::parse();
    Ok(args)
}

pub fn run(args: Args) -> MyResult<()> {
    let file_count = args.files.len();

    for (i, filename) in args.files.iter().enumerate() {
        match open(&filename) {
            Err(e) => eprintln!("{}: {}", filename , e),
            Ok(mut file) => {
                if file_count > 1 {
                    if i > 0 { println!() }

                    println!("==> {} <==", filename);
                }

                if let Some(bytes_count) = args.bytes {
                    let mut handle = file.take(bytes_count as u64);
                    let mut buf = vec![0; bytes_count];
                    let bytes_read = handle.read(&mut buf)?;

                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                    continue;
                }

                for _ in 0..args.lines {
                    let mut buf = String::new();

                    if file.read_line(&mut buf)? > 0 {
                        print!("{}", buf);
                        buf.clear();
                    }
                }
            },
        }
    }

    Ok(())
}
