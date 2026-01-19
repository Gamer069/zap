pub mod copy;
pub mod path;

use std::{io::{Error, ErrorKind}, str::FromStr};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "zap",
    about = "⚡ \x1b[1mBlazingly-fast\x1b[0m file copying tool. ⚡",
    version,
    arg_required_else_help = true
)]
pub struct Cli {
    #[arg(required = true)]
    pub src: Vec<String>,

    #[arg(required = true)]
    pub dest: String,

    #[arg(short, long)]
    pub non_recursive: bool,

    #[arg(short, long)]
    pub force: bool,

    #[arg(short, long, default_value = "w", help = "\
    Set progress display type. Options:

    i   Show progress per file (Individual)
    w   Show overall progress for the whole operation (Whole)
    n   No progress display (None)
    nv  No progress bar, but show verbose messages (NoneAndVerbose)"
    )]
    pub progress: ProgressType,

    #[arg(short, long)]
    pub strict: bool,

    #[arg(short, long)]
    pub time: bool,
}

#[derive(Parser, Debug, Clone, Copy)]
pub enum ProgressType {
    Individual,
    Whole,
    None,
    NoneAndVerbose,
}

impl FromStr for ProgressType {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i" => { Ok(ProgressType::Individual) },
            "w" => { Ok(ProgressType::Whole) },
            "n" => { Ok(ProgressType::None) },
            "nv" => { Ok(ProgressType::NoneAndVerbose) },
            _ => { Err(Error::new(ErrorKind::InvalidInput, "invalid progress type (expected: i|w|n|nv)")) }
        }
    }
}

fn main() {
    let cli = Cli::parse();
    copy::start(cli);
}
