use time::PreciseTime;

pub mod doc {
    doc_cmd!(latency,
        desc => "Returns the latency of the Discord websocket Gateway connection for this shard.");
    doc_cmd!(ping,
        desc => "Returns the round-trip time-to-contact for the Discord REST API.");

    doc_cmd!(version,
        desc => "Returns the current running version of Lain.");
}

command!(latency(ctx, msg) {
    let _ = msg.channel_id.say(&ctx.shard.lock()
        .unwrap()
        .latency()
        .map_or_else(||"N/A".to_owned(), |s| {
            format!("Shard gateway latency is **{:.3}s**", s.as_secs() as f64 + 1e-9 * s.subsec_nanos() as f64)
        }));
});

command!(ping(_ctx, msg) {
    let start = PreciseTime::now();
    let mut msg = msg.channel_id.say("Ping!").unwrap();
    let end = PreciseTime::now();

    let dt = start.to(end);

    let _ = msg.edit(|m|
        m.content(&format!("Pong! Round-trip time: **{:.2}ms**",
            dt.num_microseconds().unwrap_or(0) as f64 * 0.001)));
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

