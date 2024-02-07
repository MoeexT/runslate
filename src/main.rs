use std::env;

use clap::Parser;
use log::debug;
use runslate::{
    args::{CacheCommands, Cli, Commands},
    cache,
    translators::translate,
    utils::env_loader,
};

#[tokio::main]
async fn main() {
    // load .env for parsing arguments
    let load_env_result = env_loader::load_env_file(".env").await;

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
            if args.verbose {
                env::set_var("RUST_LOG", "trace");
            }

            // init logger
            env_logger::init();

            // log ".env loading result"
            match load_env_result {
                Ok(_) => debug!("load .env successfully."),
                Err(err) => panic!("{:#?}", err),
            }

            // log args
            debug!("{:#?}", args);

            match args.commands {
                CacheCommands::Clean => cache::cmd::clean(),
                CacheCommands::Show => cache::cmd::list(),
                CacheCommands::Purge => cache::cmd::purge(),
                CacheCommands::View { hash } => cache::cmd::view(hash),
            }
        }
        Commands::Query(args) => {
            // set verbose
            if args.verbose {
                env::set_var("RUST_LOG", "trace");
            }

            // init logger
            env_logger::init();

            // log ".env loading result"
            match load_env_result {
                Ok(_) => debug!("load .env successfully."),
                Err(err) => panic!("{:#?}", err),
            }

            // log args
            debug!("{:#?}", args);
            translate(args).await;
        }
    }
}
