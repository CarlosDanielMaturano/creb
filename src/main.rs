pub mod args;
pub mod tokens;
use args::{Args, ArgsOptions};
use clap::Parser;

fn print_line_number(index: usize) {
    print!("{: <8}", format!("{index}."));
}

fn print_match_content(mut pattern: String, mut content: String, options: ArgsOptions) {
    if options.ignore_case {
        content = content.to_lowercase();
        pattern = pattern.to_lowercase()
    }
    content.lines().enumerate().for_each(|(index, line)| {
        // only return if the line do not contain the pattern and 'options.invert' is false
        if line.contains(&pattern) ^ !options.invert {
            return
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

fn main() {
    let args = Args::parse();
    let options = match ArgsOptions::from_flags(args.flags) {
        Ok(options) => options,
        Err(err) => {
            println!("A error ocurred while parsing the flags: {err}");
            std::process::exit(0)
        }
    };
    if let Ok(content) = args.file_path.contents() {
        print_match_content(args.pattern, content, options);
    }
}
