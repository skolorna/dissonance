mod menu;

use dotenv::dotenv;
use menu::week_menu;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;

use std::env;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let menu_id = "mpi.c3c75403-6811-400a-96f8-a0e400c020ba";
    let menus = week_menu(menu_id).await.unwrap();

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.title("*This* is a title");
                e.description("This is a description");
                e.fields(menus.into_iter().map(|m| m.to_field()).collect::<Vec<_>>());
                e.footer(|f| {
                    f.text(menu_id)
                });

                // Add a timestamp for the current time
                // This also accepts a rfc3339 Timestamp
                e.timestamp(chrono::Utc::now().to_rfc3339())
            })
        })
        .await.unwrap();

    Ok(())
}
