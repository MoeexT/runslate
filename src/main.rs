use std::env;

use clap::Parser;
use log::{debug, info};
use runslate::{
    args::{CacheCommands, Cli, Commands},
    cache,
    translators::translate,
    utils::env_loader,
};

#[tokio::main]
async fn main() {
    // load .env for parsing arguments
    unsafe {
        env::set_var("RUST_LOG", "trace");
    }
    env_logger::init();
    let load_result = env_loader::load_env_file(".env").ok();
    // clear empty env, or related option will report error
    env_loader::clear_empty_env(vec![
        "RUNSLATE_TRANSLATOR",
        "RUNSLATE_SOURCE_LANG",
        "RUNSLATE_TARGET_LANG",
        "RUNSLATE_SHOW_MORE",
        "RUNSLATE_VERBOSE",
    ]);

    // parse arguments
    let cli = Cli::parse();
    let command = cli.command.unwrap_or(Commands::Query(cli.query));
    match command {
        Commands::Cache(args) => {
            // set verbose
            if !args.verbose {
                log::set_max_level(log::LevelFilter::Off);
            }
            // log args
            debug!("{:#?}", args);
            info!(
                "Load file .env: {}",
                load_result.unwrap_or("doesn't exist".to_string())
            );

            match args.commands {
                CacheCommands::Clean => cache::cmd::clean(),
                CacheCommands::Show => cache::cmd::list(),
                CacheCommands::Purge => cache::cmd::purge(),
            }
        }
        Commands::Query(args) => {
            // set verbose
            if !args.verbose {
                log::set_max_level(log::LevelFilter::Off);
            }
            // log args
            debug!("{:#?}", args);
            info!(
                "Load file .env: {}",
                load_result.unwrap_or("doesn't exist".to_string())
            );

            translate(args).await;
        }
    }
}
