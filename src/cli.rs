use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Directory to reset
    pub directory: Option<String>,

    /// Install npm dependencies as legacy
    #[arg(short, long)]
    pub legacy: bool,

    /// Delete local yalc configuration
    #[arg(short, long)]
    pub yalc: bool,

    /// Perform the only the npm install process
    #[arg(long)]
    pub install_only: bool,
}

/// Parses the command-line arguments and returns an instance of the `Args` struct.
///
/// # Returns
///
/// An instance of the `Args` struct populated with the parsed command-line arguments.
pub fn get_args() -> Args {
    return Args::parse();
}
