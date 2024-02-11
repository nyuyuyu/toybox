use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("nyuyuyu <127820811+nyuyuyu@users.noreply.github.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .default_value("-")
                .multiple(true)
                .help("Input file(s)"),
        )
        .arg(
            Arg::with_name("number_lines")
                .short("n")
                .long("number")
                .conflicts_with("number_nonblank_lines")
                .help("Number lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank_lines")
                .short("b")
                .long("number-nonblank")
                .help("Number nonblank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("file").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(r) => {
                let mut line_count = 1;
                for line in r.lines() {
                    let mut prefix = if config.number_lines || config.number_nonblank_lines {
                        format!("{:>6}\t", line_count)
                    } else {
                        String::from("")
                    };

                    let line = line?;
                    if config.number_nonblank_lines && line.is_empty() {
                        prefix = String::from("");
                    } else {
                        line_count += 1;
                    }

                    println!("{}{}", prefix, line);
                }
            }
        }
    }
    Ok(())
}
