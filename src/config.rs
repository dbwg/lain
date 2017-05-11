use std::fs::File;
use std::io::prelude::*;
use std::env;
use toml;

#[derive(Deserialize, Debug)]
pub struct Configuration {
	pub prefix: String,
	pub shards: u8,
}

#[derive(Debug)]
pub struct Secrets {
	pub token: String,
}

impl Configuration {
	pub fn from_file(f: &mut File) -> Configuration {
		let mut s = String::new();
		f.read_to_string(&mut s)
			.expect("Couldn't read from config file!");
		
		toml::from_str(&s)
			.expect("Couldn't deserialize config file!")
	}
}

impl Secrets {
	pub fn from_env() -> Secrets {
		Secrets {
			token: env::var("TOKEN")
				.expect("The TOKEN environment variable must be provided!")
		}
	}
}