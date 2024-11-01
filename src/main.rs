use poise::serenity_prelude::{self as serenity};
use std::fs;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Empty data struct
pub struct Data {}

mod groups;

struct Handler;

#[serenity::async_trait]
impl serenity::EventHandler for Handler {
    async fn reaction_add(&self, ctx: serenity::Context, reaction: serenity::Reaction) {
        let emoji = &reaction.emoji;

        if let Ok(message) = reaction.message(&ctx.http).await {
            let _ = message.react(&ctx.http, emoji.clone()).await;
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = fs::read_to_string(".token").expect("Expected token file");

    let options = poise::FrameworkOptions {
        commands: vec![groups::groups()],
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .options(options)
        .build();

    let intents = serenity::GatewayIntents::MESSAGE_CONTENT | serenity::GatewayIntents::GUILDS;

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .event_handler(Handler {})
        .await;

    client.unwrap().start().await.unwrap()
}
