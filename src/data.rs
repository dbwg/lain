use typemap::Key;
use r2d2::Pool;
use r2d2_redis::RedisConnectionManager;

pub struct RedisPool;

impl Key for RedisPool {
    type Value = Pool<RedisConnectionManager>;
}

