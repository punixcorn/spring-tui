use std::result;

use clap::Parser;

#[derive(Parser)]
#[command(name = "spring-tui")]
pub struct Args {
    /// file path to config
    #[arg(short, long)]
    file: Option<String>,

    /// default download directory
    #[arg(short, long)]
    dir: Option<String>,
}
#[derive(Default)]
pub struct CliResult {
    pub file: Option<String>,
    pub dir: Option<String>,
}

pub fn parse() -> Option<CliResult> {
    let args = Args::parse();
    let mut result: CliResult = CliResult::default();

    match (args.file, args.dir) {
        (None, None) => None,
        (Some(file), Some(dir)) => {
            result.file = Some(file);
            result.dir = Some(dir);
            Some(result)
        }
        (None, Some(dir)) => {
            result.file = None;
            result.dir = Some(dir);
            Some(result)
        }
        (Some(file), None) => {
            result.file = Some(file);
            result.dir = None;
            Some(result)
        }
    }
}
