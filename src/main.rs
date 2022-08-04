use std::{fs, error};

use poise::serenity_prelude as serenity;

mod config;
mod state;

use config::Config;
use state::State;

type Error = Box<dyn error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, State, Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config: Config = toml::from_str(&fs::read_to_string("bot.config.toml")?)?;

    let prefix_options = poise::PrefixFrameworkOptions {
        prefix: Some("-".into()),
        ..Default::default()
    };
    let framework_options = poise::FrameworkOptions {
        commands: vec![register()],
        prefix_options,
        ..Default::default()
    };

    let postgres = sqlx::postgres::PgPoolOptions::new()
        .min_connections(config.postgres.max_connections)
        .max_connections(config.postgres.max_connections)
        .connect(&config.postgres.connection_uri)
        .await?;
    
    let state = State {
        postgres,
        http: reqwest::Client::new()
    };

    let framework = poise::Framework::builder()
        .token(config.bot.token)
        .options(framework_options)
        .intents(
            serenity::GatewayIntents::all()
        )
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(state) }));

    framework.run().await?;

    Ok(())
}

#[poise::command(prefix_command, owners_only)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}