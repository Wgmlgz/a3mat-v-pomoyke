use rand::seq::SliceRandom;
use std::env;

use serenity::{
    async_trait,
    model::{
        channel::{ChannelType, GuildChannel, Message},
        gateway::Ready,
    },
    prelude::*,
};

struct Handler;

impl Handler {
    async fn move_users(count: i32, ctx: Context, msg: Message) {
        let guild = msg.guild_id.unwrap();
        let map = guild.channels(&ctx.http).await.unwrap();

        let ch: Vec<&GuildChannel> = map
            .values()
            .into_iter()
            .filter(|channel| channel.kind == ChannelType::Voice)
            .collect();

        println!("moving {} times", count);
        for move_time in 0..count {
            let selected_channel = *ch.choose(&mut rand::thread_rng()).unwrap();
            for i in &msg.mentions {
                guild
                    .move_member(&ctx.http, i.id, selected_channel)
                    .await
                    .unwrap();
            }
            println!("  moved {} time", move_time + 1);
        }
        println!("gg");
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let t: Vec<&str> = msg.content.split_whitespace().collect();
        match t[..] {
            ["move", count, ..] => {
                Handler::move_users(count.parse::<i32>().unwrap_or_else(|_| 5), ctx, msg).await
            },
            _ => {}
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
