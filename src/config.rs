use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub postgres: Postgres
}

#[derive(Deserialize)]
pub struct Bot {
    pub token: String
}

#[derive(Deserialize)]
pub struct Postgres {
    pub connection_uri: String,
    pub min_connections: u32,
    pub max_connections: u32
}