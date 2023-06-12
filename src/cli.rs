use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to reset
    pub dir: Option<String>,

    /// Install npm dependencies as legacy
    #[arg(short, long)]
    pub legacy: bool,
}

/// Parses the command-line arguments and returns an instance of the `Args` struct.
///
/// # Returns
///
/// An instance of the `Args` struct populated with the parsed command-line arguments.
pub fn get_args() -> Args {
    return Args::parse();
}
