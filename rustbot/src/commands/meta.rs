use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use serenity::cache::{Cache, Settings};
use chrono::prelude::*;







#[command]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let settings = Settings::new();
    let _cache = Cache::new_with_settings(settings);
    let guilds = ctx.cache.guilds().await.len();

    let message = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("Rust-bot info");
                    e.color(000168171);
                    e.field("Guilds", &guilds.to_string(), true);
                    e.field("Libary", "Serenity", true);
                    e.field("Version", "1.0", true);
                    e.field("Github", "WIP", true);
                    e.field("Shards", "2", true);
                    e.field("Current shard", ctx.shard_id.to_string(), true);
                    e.field("latency", format!("{} ms", (Utc::now() - msg.timestamp).num_milliseconds()), true);
                    e.footer(|f| {
                        f.text("Made by Pug#0970");

                        f
                    });

                    e
                });
                m
            }).await;

    if let Err(why) = message {
        println!("Error sending message: {:?}", why);
           }

    Ok(())
}

#[command]
async fn rustping(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.channel_id.send_message(&ctx.http, |m| {
                m.embed(|e| {
                    e.title("latency");
                    e.color(000168171);
                    e.description(format!("{} ms", (Utc::now() - msg.timestamp).num_milliseconds()));

                    e
                });
                m
            }).await;

    if let Err(why) = message {
        println!("Error sending message: {:?}", why);
           }

    Ok(())
}
