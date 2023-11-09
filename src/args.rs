use clap::{Parser, Subcommand, Args};

use crate::translators::QueryArgs;

#[derive(Debug, Parser)]
#[command(name = "")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[command(flatten)]
    pub query: QueryArgs,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Clean(CleanArgs),
    Query(QueryArgs),
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct CleanArgs {
    /// [bool] Print debug details
    #[arg(short = 'v', long, env = "RUNSLATE_VERBOSE")]
    pub verbose: bool,
}
