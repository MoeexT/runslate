use clap::{Args, Parser, Subcommand};

use crate::translators::QueryArgs;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
#[command(subcommand_precedence_over_arg = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[command(flatten)]
    pub query: QueryArgs,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage cache
    Cache(CacheArgs),

    /// Default command
    Query(QueryArgs),
}

#[derive(Debug, Args)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub commands: CacheCommands,

    /// [bool] Print debug details
    #[arg(short = 'v', long, env = "RUNSLATE_VERBOSE")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum CacheCommands {
    /// Clean cache
    Clean,

    /// Show cache, alias: list
    #[command(alias="list")]
    Show,

    /// Remove expired cache
    Purge,

    /// View cache content
    View {
        hash: String,
    },
}
