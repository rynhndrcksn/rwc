use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    pub files: Vec<PathBuf>,

    #[arg(short, long)]
    pub bytes: bool,

    #[arg(short, long)]
    pub lines: bool,

    #[arg(short, long)]
    pub words: bool,

    #[arg(short, long)]
    pub chars: bool,
}
