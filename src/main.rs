extern crate serde_json;
extern crate serenity;
extern crate tokio;

use serde_json::json;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = include_str!("./token");
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct Handler;

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.mentions_me(&context).await.unwrap_or(false) {
            let map = json!({
                "content": "It works!",
                "message_reference": {
                    "message_id": *msg.id.as_u64()
                }
            });

            let _ = context.http.send_message(msg.channel_id.0, &map).await;
        }
    }
}
