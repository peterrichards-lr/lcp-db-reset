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
    /// Reset the database schema for a given project/environment
    Reset {
        /// The project ID (e.g., lfr-prj)
        #[arg(short, long)]
        project: String,

        /// The environment (e.g., dev, uat, prd)
        #[arg(short, long)]
        environment: String,

        /// The service name (default: liferay)
        #[arg(short, long, default_value = "liferay")]
        service: String,

        /// Specific instance ID (optional, found automatically if missing)
        #[arg(short, long)]
        instance: Option<String>,

        /// Database password (optional, prompted if missing)
        #[arg(short = 'P', long)]
        password: Option<String>,

        /// Skip confirmation prompt
        #[arg(long)]
        force: bool,
    },
}
