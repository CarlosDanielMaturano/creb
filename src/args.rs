use clap::Parser;
use std::io::{Error, ErrorKind};

#[derive(Parser, Debug)]
pub struct Args {
    pub pattern: String,
    pub file_path: std::path::PathBuf,
    pub flags: Vec<String>,
}

pub struct ArgsOptions {
    pub invert: bool,
    pub colorize: bool,
    pub line_numbers: bool,
}

fn parse_flags(flags: &Vec<String>) -> Vec<String> {
    let parsed_flags = flags
        .iter()
        .flat_map(|flag| {
            if flag.is_empty() {
                return vec![String::new()];
            }
            if flag.matches("-").count() >= 2 {
                return vec![flag[2..].to_string()];
            }
            return flag[1..]
                .chars()
                .map(String::from)
                .collect();
        })
        .collect::<Vec<String>>();
    parsed_flags
}

impl ArgsOptions {
    pub fn from_flags(flags: Vec<String>) -> Result<ArgsOptions, Error> {
        let mut options = ArgsOptions {
            invert: false,
            colorize: false,
            line_numbers: false,
        };

        for flag in parse_flags(&flags) {
            match flag.as_str() {
                "v" | "invert" => options.invert = true,
                "c" | "color" => options.colorize = true,
                "n" | "numbers" => options.line_numbers = true,
                _ => return Err(Error::new(ErrorKind::Other, format!("Unknow flag: {flag}"))),
            }
        }

        Ok(options)
    }
}
