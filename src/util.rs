use serenity::client::rest;

pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

pub fn commit() -> &'static str {
    include_str!(concat!(env!("OUT_DIR"), "/commit-info.txt"))
}

pub fn recommended_shards() -> u64 {
    let res = rest::get_bot_gateway().expect("Couldn't get a BotGateway!");

    res.shards as u64
}

const B_PER_MB: u64 = 1024 * 1024;
pub fn bytes_to_mb(b: u64) -> f64 {
    (b as f64)/(B_PER_MB as f64)
}
