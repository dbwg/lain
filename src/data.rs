use std::sync::Mutex;
use typemap::{ShareMap, Key};
use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;

pub struct RedisPool;

impl Key for RedisPool {
    type Value = Pool<RedisConnectionManager>;
}


/// Gets a pooled connection to Redis, via a passed reference to `ctx.data`.
///
/// # Examples
///
/// ```
/// let conn = ::data;:get_redis_conn(&ctx.data);
/// ```
pub fn get_redis_conn(data: &Mutex<ShareMap>) -> PooledConnection<RedisConnectionManager> {
    let mut data = data.lock().unwrap();
    data.get_mut::<RedisPool>().unwrap().get().unwrap()
}
