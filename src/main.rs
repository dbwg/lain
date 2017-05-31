#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate serenity;

extern crate psutil;
extern crate env_logger;
extern crate toml;
extern crate time;
extern crate dotenv;
extern crate typemap;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

mod config;
mod data;
mod util;
mod commands;

use std::cmp;
use std::default::Default;
use std::fs::File;
use serenity::prelude::*;
use serenity::ext::framework::{DispatchError, help_commands};
use r2d2_redis::RedisConnectionManager;
use dotenv::dotenv;

use config::{Configuration, Secrets};
use data::RedisPool;

fn create_redis_pool(redis_url: &str) -> r2d2::Pool<RedisConnectionManager> {
    let poolconfig = Default::default();
    let manager = RedisConnectionManager::new(redis_url)
     .expect("Error creating Redis connection manager");

    r2d2::Pool::new(poolconfig, manager).unwrap()
}

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


    let redis_pool = create_redis_pool(&*config.redis_url.clone().expect("A Redis URL must be provided"));
    info!("Successfully created Redis connection pool (size: {}, url: {})",
        redis_pool.config().pool_size(), config.redis_url.unwrap());

    let mut client = Client::login(&secrets.token);

    // pick the number of shards we'll use
    let recommended_shards = util::recommended_shards();
    info!("Discord recommends N={} shards", recommended_shards);
    config.shards = Some(match config.shards {
        Some(n) => cmp::max(n, recommended_shards), // if we're using less than recommended, autobump to what discord recommends
        None => recommended_shards,
    });

    // Insert stuff into `ctx.data`.
    // Note: We wrap this in its own block so that when the block ends,
    // `data` is dropped and thus the `Mutex` is unlocked.
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<RedisPool>(redis_pool.clone());
    }

    client.with_framework(|f| f
        .configure(|c| c
            .on_mention(true)
            .prefix("~"))
        .on_dispatch_error(|_ctx, msg, error| {
            match error {
                DispatchError::RateLimited(wait_s) => {
                    let _ = msg.channel_id.say(&format!("Try again in **{}s**.", wait_s));
                },
                _ => {}, // drop all other errors
            }
        })
        .command("help", |c| c
            .help_available(false)
            .exec_help(help_commands::with_embeds))
        .group("meta", |g| g
            .command("latency", |c| c
                .desc(commands::meta::doc::latency::desc)
                .exec(commands::meta::latency))
            .command("ping", |c| c
                .desc(commands::meta::doc::ping::desc)
                .exec(commands::meta::ping))
            .command("version", |c| c
                .desc(commands::meta::doc::version::desc)
                .exec(commands::meta::version))
            .command("stats", |c| c
                .desc(commands::meta::doc::stats::desc)
                .exec(commands::meta::stats))));

    client.on_ready(|_ctx, ready| {
        if let Some(s) = ready.shard {
            info!("Logged in as '{}' on shard {}/{}",
                ready.user.name,
                s[0] + 1, // convert the index to a ordinal
                s[1]);
        } else {
            info!("Logged in as '{}'", ready.user.name);
        }
    });

    info!("Connecting to Discord with {} shard(s)", config.shards.unwrap());
    if let Err(e) = client.start_shards(config.shards.unwrap()) {
        error!("Discord client error: {:?}", e);
    }
}
