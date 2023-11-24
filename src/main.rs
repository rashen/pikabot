use std::fs;

use once_cell::sync::OnceCell;
use serenity::async_trait;
use serenity::model::channel::{Message, Reaction, ReactionType};
use serenity::model::gateway::Ready;
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::user::CurrentUser;
use serenity::prelude::*;
use std::sync::RwLock;

struct Handler;

static USER: OnceCell<RwLock<CurrentUser>> = OnceCell::new();

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message is received - the
    // closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple events can be dispatched
    // simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an authentication error, or lack
            // of permissions to post in the channel, so log to stdout when some error happens,
            // with a description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        } else if msg.content.to_ascii_lowercase().contains("pika") {
            if let Some(this_user) = USER.get() {
                let user_id = this_user.read().unwrap().id;
                if user_id != msg.author.id {
                    println!("Received {msg:?}");
                    if let Err(why) = msg.channel_id.say(&ctx.http, "Pikachuu!").await {
                        println!("Error sending message: {why:?}");
                    }
                }
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a shard is booted, and
    // a READY payload is sent by Discord. This payload contains data like the current user's guild
    // Ids, current user data, private channels, and more.
    //
    // In this case, just print what the current user's username is.
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

            for (_, channel) in channels.iter() {
                if channel.name() == "general" {
                    if let Err(why) = channel.say(&ctx.http, "Pika-pika!").await {
                        println!("Error sending message: {why:?}");
                    }
                }
            }
        }
    }

    async fn reaction_add(&self, ctx: Context, add_reaction: Reaction) {
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
    let mut token = fs::read_to_string(".token").expect("Expected token file");
    let token = token.trim();
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
