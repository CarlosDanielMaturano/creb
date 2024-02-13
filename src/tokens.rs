use colored::Colorize;

pub fn split_line_tokens(line: String) -> Vec<String> {
    line.split(" ")
        .map(|token| token.to_owned() + " ")
        .collect::<Vec<String>>()
}

// only colorize the pattern in the token
pub fn colorize_token_with_pattern(pattern: &String, token: &String) -> String {
    let begin_pattern_pos = token.find(pattern).unwrap();
    let end_pattern_pos = begin_pattern_pos + pattern.len();
    let matched_pattern = &token[begin_pattern_pos..end_pattern_pos];
    format!(
        "{}{}{}",
        &token[..begin_pattern_pos],
        matched_pattern.red(),
        &token[end_pattern_pos..]
    )
}
