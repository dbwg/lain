use serenity::client::rest;

/// Gets the number of shards Discord recommends we connect with.
pub fn recommended_shards() -> u64 {
    let res = rest::get_bot_gateway().expect("Couldn't get a BotGateway!");

    res.shards as u64
}

/// Converts a number of bytes to mibibytes/binary megabytes.
const B_PER_MB: u64 = 1024 * 1024;
pub fn bytes_to_mb(b: u64) -> f64 {
    (b as f64)/(B_PER_MB as f64)
}

/// Returns the version of the bot, as specified in `Cargo.toml`.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Returns the current Git commit hash of the bot repo.
///
/// This is obtained from a file created by the Cargo build script.
pub fn commit() -> (&'static str, &'static str) {
    let c = include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"));
    let mut s = c.split_whitespace();
    (s.next().unwrap_or_default(), s.next().unwrap_or_default())
}
