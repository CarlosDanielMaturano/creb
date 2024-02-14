pub mod args;
pub mod tokens;
use args::{Args, ArgsOptions};
use clap::Parser;

fn read_file_content(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let file_content = std::fs::read_to_string(path)?;
    Ok(file_content)
}

fn print_line_number(index: usize) {
    print!("{: <8}", format!("{index}."));
}

fn print_match_content(pattern: String, content: String, options: ArgsOptions) {
    content.lines().enumerate().for_each(|(index, line)| {
        if !line.contains(&pattern) {
            return;
        }
        if options.line_numbers {
            print_line_number(index)
        }
        // each token is a sequential character string without any whitspace that has been
        // found on the current line
        let tokens = tokens::split_line_tokens(line.to_string()).into_iter();
        // print the token list as a new line
        tokens.for_each(|token| {
            let mut token = token;
            if options.colorize && token.contains(&pattern) {
                token = tokens::colorize_token_with_pattern(&pattern, &token);
            }
            print!("{token}");
        });
        println!()
    });
}

fn print_invert_content(pattern: String, content: String, options: ArgsOptions) {
    content.lines().enumerate().for_each(|(index, line)| {
        if line.contains(&pattern) {
            return;
        }
        if options.line_numbers {
            print_line_number(index);
        }
        println!("{line}");
    });
}

fn main() {
    let args: Args = Args::parse();

    let options = match ArgsOptions::from_flags(args.flags) {
        Ok(options) => options,
        Err(err) => {
            println!("A error ocurred while parsing the flags: {err}");
            std::process::exit(0)
        }
    };
    if let Ok(file_content) = read_file_content(&args.file_path) {
        if options.invert {
            return print_invert_content(args.pattern, file_content, options);
        }
        print_match_content(args.pattern, file_content, options);
    }
}
