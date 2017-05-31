use std::fs::File;
use std::io::prelude::*;
use std::env;
use ::errors::*;
use toml;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    pub prefix: String,
    pub shards: Option<u64>,

    pub postgres_url: Option<String>,
    pub redis_url: Option<String>,
}

#[derive(Debug)]
pub struct Secrets {
    pub token: String,
}

impl Configuration {
    pub fn from_file(f: &mut File) -> Result<Configuration> {
        let mut s = String::new();
        f.read_to_string(&mut s)
            .chain_err(|| "Error reading configuration file to string")?;

        Ok(toml::from_str(&s).chain_err(|| "Error deserializing Configuration")?)
    }

    pub fn overlay_env(self: &mut Configuration) {
        self.redis_url = env::var("REDIS_URL").ok();
        self.postgres_url = env::var("POSTGRES_URL").ok();
    }
}

impl Secrets {
    pub fn from_env() -> Result<Secrets> {
        Ok(Secrets {
            token: env::var("TOKEN").chain_err(|| "TOKEN environment var must be set")?,
        })
    }
}
