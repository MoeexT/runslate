# runslate

A shell dictionary created by Rust.

## :alien: API provider

+ [youdao](https://ai.youdao.com/product-fanyi-text.s)
+ [google](https://translate.google.com/)

## :construction: Installation

### Binary

Just download the binary file and put it into a directory which has add to `path`.

### From source

1. Visit [rust-lang.org](https://www.rust-lang.org/tools/install) and select a suitable way to install Rust.
2. Clone this repo and run `cargo build [--release]`.

## :page_with_curl: Usage

1. The best way to use `runslate` is put it into a dir contained by `env path` and rename it into a shorter name so you can lookup words more conveniently.
2. Type `runslate -h` for usage details, here are some options for example:
   + `-t, --translator` to select translator(API provider).
   + `-s, --source-lang` to set source language.
   + `-d, --target-lang` to set target language.
   + `-v, --verbose` show debug logs.
   + ...
3. `runslate` provides some `env-vars` for configuration, in `.env`:
   1. Use `RUNSLATE_TRANSLATOR` to pick translator. Available values: [`youdao`, `google`].
   2. Check env template file `.env` for more envs.
4. How does `env` work?
   1. First, runslate will try to read `.env` in current directory. If failed, go to step 2.
   2. Second, parse options:
      1. Some options are related to specified `env-vars`, if they were found, use them first.
      2. Use options' default values(if given).
      3. If there is not `env-var` neither default value, an error is reported.
   3. As for some circumstances like env conflicting, not tested yet.

## :hammer: Crates used

+ [clap](https://docs.rs/clap/latest/clap/): parse arguments.
+ [reqwest](https://docs.rs/reqwest/latest/reqwest/): http request.
+ [serde_json](https://docs.rs/serde_json/latest/serde_json/): json parse.
+ [env_logger](https://docs.rs/env_logger/latest/env_logger/): logging.
+ [dotenv](https://docs.rs/dotenv/latest/dotenv/): load env.