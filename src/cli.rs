use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Command-line arguments structure
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Output format (text is default)
    #[arg(short='f', long, value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    /// Input file (reads from stdin if not provided)
    #[arg(short='i', long, value_name = "INPUT", required = false)]
    pub input: Option<PathBuf>,

    /// Output file (writes to stdout if not provided)
    /// If the output file already exists, it will be overwritten
    #[arg(short='o', long, value_name = "OUTPUT", required = false)]
    pub output: Option<PathBuf>,
}

/// Supported output formats
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    Text,
    Html,
}

/// Parse command-line arguments
pub fn get_args() -> Args {
    Args::parse()
}
