#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate serenity;
extern crate env_logger;
extern crate toml;
extern crate dotenv;

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

	let mut client = Client::login(&secrets.token);

	info!("Connecting to Discord with {} shard(s)", config.shards);
	if let Err(e) = client.start() {
		error!("Discord client error: {:?}", e);
	}
}
