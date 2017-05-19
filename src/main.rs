#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate env_logger;
extern crate toml;
extern crate dotenv;
extern crate redis;

mod config;

use std::fs::File;
use serenity::Client;
use dotenv::dotenv;

use config::{Configuration, Secrets};

fn main() {
    dotenv().ok();
    env_logger::init().unwrap();

    info!("Loading secrets");
    let secrets = Secrets::from_env();

    let config_path = "config.toml";
    info!("Loading config from {}", config_path);
    let mut config_file = File::open(config_path)
        .expect("Error opening config file!");
    let mut config = Configuration::from_file(&mut config_file);
    config.overlay_env();


    // pick the number of shards we'll use
    let recommended_shards = util::recommended_shards();
    info!("Discord recommends N={} shards", recommended_shards);
    config.shards = Some(match config.shards {
        Some(n) => cmp::max(n, recommended_shards), // if we're using less than recommended, autobump to what discord recommends
        None => recommended_shards,
    });



    info!("Connecting to Discord with {} shard(s)", config.shards.unwrap());
    if let Err(e) = client.start_shards(config.shards.unwrap()) {
        error!("Discord client error: {:?}", e);
    }
}
