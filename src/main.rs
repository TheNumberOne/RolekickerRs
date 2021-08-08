use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group,
    },
};

use rbatis::rbatis::Rbatis;

use std::env;
use lazy_static::lazy_static;

lazy_static! {
  // Rbatis is thread-safe, and the runtime method is Send+Sync. there is no need to worry about thread contention
  static ref RB:Rbatis = Rbatis::new();
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    env_logger::init();
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");

    let db_url = env::var("DATABASE_URL").expect("database url");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    RB.link(&db_url).await.unwrap();

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}