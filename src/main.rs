use std::env;

use clap::{arg, Parser};
use log::{debug, error, info, warn};
use runslate::{
    translators::{
        google::Google, load_cache, save_cache, youdao::Youdao, Lang, Translator, Translators,
    },
    utils::env_loader,
};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// [enum] Translator
    #[arg(
        short = 't',
        long,
        default_value = "youdao",
        env = "RUNSLATE_TRANSLATOR"
    )]
    translator: Translators,

    /// [enum] Source language
    #[arg(
        short = 's',
        long,
        default_value = "auto",
        env = "RUNSLATE_SOURCE_LANG"
    )]
    source_lang: Lang,

    /// [enum] Target language
    #[arg(short = 'd', long, default_value = "zh", env = "RUNSLATE_TARGET_LANG")]
    target_lang: Lang,

    /// [bool] Print more translation info
    #[arg(short, long, default_value = "true", env = "RUNSLATE_SHOW_MORE")]
    more: bool,

    /// [bool] Decides if to use cache
    #[arg(short = 'n', long, default_value = "false", env = "RUNSLATE_NO_CACHE")]
    no_cache: bool,

    /// [bool] Print debug details
    #[arg(short = 'v', long, env = "RUNSLATE_VERBOSE")]
    verbose: bool,

    /// [strings] Words to translate
    #[arg(num_args=1.., required = true)]
    words: Vec<String>,
}

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
    let args = &Args::parse();

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

    let cache_time = env_loader::load_or_default("RUNSLATE_CACHE_TIME", "300");
    let cache_time = cache_time.parse::<u64>().or::<u64>(Ok(300)).unwrap();

    debug!("Cache time: {}", cache_time);

    let words = args.words.join(" ");
    match args.translator {
        Translators::Google => {
            if !args.no_cache {
                if let Ok(response) =
                    load_cache(&words, &args.target_lang, &args.translator, cache_time)
                {
                    info!("Load querying result from cache successfully.");
                    Google::show(&response.data, args.more);
                    return;
                }
                warn!("Try load cache failed.")
            }

            match Google::translate(&words, &args.source_lang, &args.target_lang).await {
                Ok(response) => {
                    debug!("{:#?}", &response);
                    Google::show(&response, args.more);
                    if !args.no_cache {
                        save_cache(&words, &args.target_lang, &args.translator, response);
                    }
                }
                Err(err) => error!("{:#?}", err),
            }
        }
        Translators::Youdao => {
            if !args.no_cache {
                if let Ok(response) =
                    load_cache(&words, &args.target_lang, &args.translator, cache_time)
                {
                    info!("Load querying result from cache successfully.");
                    Youdao::show(&response.data, args.more);
                    return;
                }
                warn!("Try load cache failed.")
            }

            match Youdao::translate(&words, &args.source_lang, &args.target_lang).await {
                Ok(response) => {
                    debug!("{:#?}", &response);
                    Youdao::show(&response, args.more);
                    if !args.no_cache {
                        save_cache(&words, &args.target_lang, &args.translator, response);
                    }
                }
                Err(err) => error!("{:#?}", err),
            }
        }
    }
}
