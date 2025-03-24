use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "timber")]
#[clap(about = "Timber: Fell Your Logs Fast", long_about = None)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    /// Log file to analyze
    pub file: String,

    /// Pattern to search for
    #[clap(short, long)]
    pub chop: Option<String>,

    /// Filter by log level (ERROR, WARN, INFO, etc.)
    #[clap(short, long)]
    pub level: Option<String>,

    /// Show time-based trends
    #[clap(long)]
    pub trend: bool,

    /// Show summary statistics
    #[clap(long)]
    pub stats: bool,

    /// Output results in JSON format
    #[clap(long)]
    pub json: bool,

    /// Number of top error types to show (default: 5)
    #[clap(long, default_value = "5")]
    pub top_errors: usize,

    /// Show unique messages in the output
    #[clap(long)]
    pub show_unique: bool,

    /// Force parallel processing (default: auto-detect based on file size)
    #[clap(long)]
    pub parallel: bool,

    /// Force sequential processing (overrides parallel)
    #[clap(long)]
    pub sequential: bool,

    /// Only output the total log count (fast mode)
    #[clap(long)]
    pub count: bool,
}
