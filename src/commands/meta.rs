use psutil;
use serenity::client::CACHE;
use time::PreciseTime;
use ::util::bytes_to_mb;

pub mod doc {
    doc_cmd!(latency,
        desc => "Returns the latency of the Discord websocket Gateway connection for this shard.");
    doc_cmd!(ping,
        desc => "Returns the round-trip time-to-contact for the Discord REST API.");

    doc_cmd!(version,
        desc => "Returns the current running version of Lain.");

    doc_cmd!(stats,
        desc => "Retrieve stats about Lain, including memory utilization, commands/min, and handlers fired/min.");
}

command!(latency(ctx, msg) {
    let _ = msg.channel_id.say(&ctx.shard.lock()
        .unwrap()
        .latency()
        .map_or_else(||"N/A".to_owned(), |s| {
            format!("Shard gateway latency is **{}.{}s**", s.as_secs(), s.subsec_nanos())
        }));
});

command!(ping(_ctx, msg) {
    let start = PreciseTime::now();
    let mut msg = msg.channel_id.say("Ping!").unwrap();
    let end = PreciseTime::now();

    let dt = start.to(end);

    let _ = msg.edit(|m|
        m.content(&format!("Pong! **{}.{}ms**",
            dt.num_milliseconds(),
            dt.num_microseconds().unwrap_or(0))));
});

command!(version(_ctx, msg) {
    let commit = ::util::commit();
    let _ = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .title("__LainBot__")
            .color(0xb997ce)
            .field(|f| f.name("version").value(::util::version()))
            .field(|f| f.name("commit").value(
                &format!("**hash:** {}\n**date**: {}", commit.0, commit.1)))));
});

command!(stats(_ctx, msg) {
    let process = match psutil::process::Process::new(psutil::getpid()) {
        Ok(p) => p,
        Err(e) => {
            error!("Error getting processes: {:?}", e);

            let _ = msg.channel_id.say(":cry: Error getting process stats.");

            return Ok(());
        }
    };

    let memory = match process.memory() {
        Ok(m) => m,
        Err(e) => {
            error!("Error getting process memory: {:?}", e);

            let _ = msg.channel_id.say(":cry: Error getting process memory status.");

            return Ok(());
        }
    };

    let mem_total = bytes_to_mb(memory.size);
    let mem_rss = bytes_to_mb(memory.resident);
    let memory = format!("**total:** {:.3}\n**resident:** {:.3}", mem_total, mem_rss);

    let guilds = CACHE.read().unwrap().guilds.len();

    let _ = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .title("Statistics")
            .field(|f| f.name("Memory").value(&memory))
            .field(|f| f.name("Guilds").value(&guilds.to_string()))));
});
