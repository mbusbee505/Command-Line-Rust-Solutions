use clap::Parser;
use anyhow::Result;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug, Parser)]
#[command(author, version, about)]

/// Rust version of `head`
struct Args {

    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(
        short('n'),
        long("lines"),
        conflicts_with("bytes"),
        help = "Number of lines",
        default_value = "10",
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(
        short('c'),
        long("bytes"),
        help = "Number of bytes",
        conflicts_with("lines"),
        value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();
    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{filename}: {err}"),
            Ok(mut reader) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==",
                    if file_num > 0 { "\n" } else { "" }
                    );
                }
                // Use Bytes
                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = reader.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                }
                
                // Use Lines
                else {
                    let mut line = String::new();
                    let mut counter = 0;
                    loop {
                        let bytes_read = reader.read_line(&mut line)?;
                        if bytes_read == 0 {
                            break;
                        }
                    print!("{line}");
                    counter += 1;
                    
                    line.clear();
                    if counter == args.lines {
                        break;
                    }
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}