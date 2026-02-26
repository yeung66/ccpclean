use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "ccpclean",
    version,
    about = "Scan and clean up orphaned local web service processes",
    long_about = None
)]
pub struct Cli {
    /// Loose mode: show all processes listening on local ports (default: strict - dev runtimes only)
    #[arg(short = 'a', long = "all")]
    pub all: bool,

    /// Filter by specific port
    #[arg(short = 'p', long = "port")]
    pub port: Option<u16>,

    /// Non-interactive: print list and exit
    #[arg(long = "no-tui")]
    pub no_tui: bool,
}
