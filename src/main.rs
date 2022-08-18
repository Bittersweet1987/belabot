use std::env;

use anyhow::Result;

use not_yet_named_bot::{Bot, Settings};
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

#[tokio::main]
async fn main() -> Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "not_yet_named_bot=info");
    }

    if cfg!(windows) {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_ansi(false)
            .init();
    } else {
        tracing_subscriber::fmt::init();
    }

    let config = match Settings::load("config.json") {
        Ok(c) => c,
        Err(_) => Settings::ask_for_settings().await?,
    };

    let bot = Bot::new(config).await?;

    // There is no way to recover when any of these stop, so stop the program
    tokio::select! {
        _ = bot.bb_msg_handle => {}
        _ = bot.tw_msg_handle => {}
    };

    Ok(())
}
