#![recursion_limit = "1024"]

#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
#[macro_use] extern crate serenity;
#[macro_use] extern crate error_chain;

extern crate psutil;
extern crate env_logger;
extern crate toml;
extern crate time;
extern crate dotenv;
extern crate typemap;
extern crate r2d2;
extern crate r2d2_redis;
extern crate redis;

mod errors { error_chain!{} }
mod config;
mod data;
mod util;
mod commands;

use std::cmp;
use std::process;
use std::fs::File;
use std::default::Default;
use std::collections::HashSet;
use serenity::prelude::*;
use serenity::ext::framework::{DispatchError, help_commands};
use redis::Commands;
use r2d2_redis::RedisConnectionManager;
use dotenv::dotenv;

use config::{Configuration, Secrets};
use data::RedisPool;
use errors::*;

fn create_redis_pool(redis_url: &str) -> Result<r2d2::Pool<RedisConnectionManager>> {
    let poolconfig = Default::default();
    let manager = RedisConnectionManager::new(redis_url)
        .chain_err(|| "Unable to create a Redis connection manager")?;
    let pool = r2d2::Pool::new(poolconfig, manager)
        .chain_err(|| "Unable to create a r2d2 Pool for Redis connections")?;

    Ok(pool)
}

fn run() -> Result<()> {
    dotenv().ok();
    env_logger::init().unwrap();

    // -- Load secrets and configuration.
    info!("Loading secrets");
    let secrets = Secrets::from_env()?;

    let config_path = "config.toml";
    info!("Loading config from {}", config_path);
    let mut config_file = File::open(config_path)
        .chain_err(|| "Unable to open config file")?;
    let mut config = Configuration::from_file(&mut config_file)?;
    config.overlay_env();

    // -- Create database connections/pools.
    let redis_pool = create_redis_pool(
        &*config.redis_url.clone().ok_or("A Redis URL must be provided")?)?;
    info!("Successfully created Redis connection pool (size: {}, url: {})",
        redis_pool.config().pool_size(), config.redis_url.unwrap());

    let mut client = Client::login(&secrets.token);

    // Recalculate number of shards we'll use.
    //   n_shards = max(n_configured, n_recommended)
    let recommended_shards = util::recommended_shards();
    info!("Discord recommends N={} shards", recommended_shards);
    config.shards = Some(match config.shards {
        Some(n) => cmp::max(n, recommended_shards), // if we're using less than recommended, autobump to what discord recommends
        None => recommended_shards,
    });

    let owners = {
        let info = serenity::client::rest::get_current_application_info()
            .chain_err(|| "Error getting application info via serenity::client::rest")?;
        let mut s = HashSet::new();
        s.insert(info.owner.id);

        s
    };

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
            .owners(owners)
            .prefixes(vec!["~", "lain~"]))
        .before(move |ctx, msg, cmd| {
            let conn = data::get_redis_conn(&ctx.data);
            let _: i64 = conn.incr("usagecount:command:{}".to_owned() + cmd, 1).unwrap();

            info!("Command '{}' used by {}#{}", cmd, msg.author.name, msg.author.discriminator);

            true
        })
        .on_dispatch_error(|_ctx, msg, error| {
            match error {
                DispatchError::RateLimited(wait_s) => {
                    info!("User {}#{} exceeded rate limits with command call '{}'; making them wait {}s",
                        msg.author.name, msg.author.discriminator, msg.content, wait_s);
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
                .exec(commands::meta::version)))
        .command("stats", |c| c
            .owners_only(true).help_available(false)
            .exec(commands::owner::stats)));

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
    client.start_shards(config.shards.unwrap())
        .chain_err(|| "Error starting shards")?;

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        error!("{}", e);
        for e in e.iter().skip(1) {
            error!("  caused by: {}", e);
        }

        process::exit(1);
    }
}
