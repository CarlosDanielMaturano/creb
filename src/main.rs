pub mod args;
pub mod tokens;
use args::Args;
use clap::Parser;

fn read_file_content(path: &std::path::PathBuf) -> Result<String, std::io::Error> {
    let file_content = std::fs::read_to_string(path)?;
    Ok(file_content)
}

fn match_flags(flags: &str) {
    dbg!("{}", flags);
    match flags {
        "-v" => {}
        _ => {}
    }
}

fn print_match_content(pattern: String, content: String, colorize: bool) {
    content.lines().enumerate().for_each(|(index, line)| {
        if !line.contains(&pattern) {
            return;
        }
        // each token is a sequential character string without any whitspace that has been
        // found on the current line
        let tokens = tokens::split_line_tokens(line.to_string()).into_iter();

        print!("{: <8}", format!("{index}."));
        // print the token list as a new line
        tokens.for_each(|token| {
            let mut token = token;
            if !token.contains(&pattern) {
                return
            }
            if colorize {
                token = tokens::colorize_token_with_pattern(&pattern, &token);
            }
            print!("{token}");
        });
        println!()
    })
}

fn main() {
    let args: Args = Args::parse();
    if let Ok(file_content) = read_file_content(&args.file_path) {
        match_flags(&args.flags);
        print_match_content(args.pattern, file_content, true);
    }
}
