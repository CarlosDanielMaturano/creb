use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    pub pattern: String,
    pub file_path: std::path::PathBuf,
    pub flags: String,
}
