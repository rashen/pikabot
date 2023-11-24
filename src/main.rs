use std::fs;

use once_cell::sync::OnceCell;
use serenity::async_trait;
use serenity::http::Http;
use serenity::model::channel::{Message, Reaction};
use serenity::model::gateway::Ready;
use serenity::model::id::ChannelId;
use serenity::model::user::CurrentUser;
use serenity::prelude::*;
use std::sync::RwLock;

struct Handler;

static USER: OnceCell<RwLock<CurrentUser>> = OnceCell::new();

async fn send_message(http: impl AsRef<Http>, channel: ChannelId, msg: impl std::fmt::Display) {
    match channel.say(http, msg).await {
        Ok(_) => {}
        Err(why) => {
            println!("Error sending message: {why:?}");
        }
    };
}

#[async_trait]
impl EventHandler for Handler {
    // Called whenever a message is received
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            send_message(&ctx.http, msg.channel_id, "Pong!").await;
        } else if msg.content.to_ascii_lowercase().contains("pika") {
            if let Some(this_user) = USER.get() {
                let user_id = this_user.read().unwrap().id;
                if user_id != msg.author.id {
                    send_message(&ctx.http, msg.channel_id, "Pikachuu!").await;
                }
            }
        }
    }

    // Called on ready
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        USER.set(RwLock::new(ready.user.clone()))
            .expect("Could not write current user");

        for guild in ready.guilds {
            let guild = guild.id;
            let channels = match guild.channels(&ctx.http).await {
                Ok(map) => map,
                Err(why) => {
                    println!("Error getting channels: {why:?}");
                    return;
                }
            };

            for (channel_id, channel) in channels.iter() {
                if channel.name() == "general" {
                    send_message(&ctx.http, *channel_id, "Pika-pika!").await;
                }
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
        // Add same reaction again
        let emoji = add_reaction.emoji;
        let channel = add_reaction.channel_id;
        let message_id = add_reaction.message_id;
        if let Ok(message) = channel.message(&ctx.http, message_id).await {
            if let Err(why) = message.react(&ctx.http, emoji).await {
                println!("Error reacing to message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = fs::read_to_string(".token").expect("Expected token file");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will automatically prepend
    // your bot token with "Bot ", which is a requirement by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        // .application_id(APP_ID)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform exponential backoff until
    // it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
