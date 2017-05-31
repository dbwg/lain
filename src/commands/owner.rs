use psutil;
use serenity::client::CACHE;

use ::util::bytes_to_mb;

pub mod doc {
    doc_cmd!(stats,
        desc => "Retrieve stats about Lain, including memory utilization, commands/min, and handlers fired/min.");
}

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
    let memory = format!("**total:** {:.3}MiB\n**resident:** {:.3}MiB", mem_total, mem_rss);

    let guilds = CACHE.read().unwrap().guilds.len();

    let _ = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .color(0xb997ce)
            .title("__Statistics__")
            .field(|f| f.name("Memory").value(&memory))
            .field(|f| f.name("Guilds").value(&guilds.to_string()))));
});
