use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "lcp-db-reset")]
#[command(about = "Reset Liferay Cloud (LCP) database schema", long_about = None)]
pub struct App {
    #[command(subcommand)]
    pub command: AppCommands,
}

#[derive(Subcommand)]
pub enum AppCommands {
    /// Reset the database schema for a given project
    Reset {
        /// The project ID (e.g., lctpjr20260310-prd)
        #[arg(short, long)]
        project: String,

        /// Database password (optional, prompted if missing)
        #[arg(short = 'P', long)]
        password: Option<String>,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,

        /// Specific service name (default: liferay)
        #[arg(short, long, default_value = "liferay")]
        service: String,
    },
}
