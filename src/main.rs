extern crate peg;
extern crate rand;
extern crate regex;
extern crate serde_json;
extern crate serenity;
extern crate tokio;

mod parser;
mod runtime;

use regex::Regex;
use serde_json::json;
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::Client;

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = include_str!("./token");
    let mut client = Client::builder(token)
        .event_handler(Handler::default())
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

struct Handler {
    mention_pattern: Regex,
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            mention_pattern: Regex::new(r"<@!\d+>").unwrap(),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.mentions_me(&context).await.unwrap_or(false) {
            // メンションを削除
            let content = self.mention_pattern.replace_all(&msg.content, "");
            if let Ok(mut value) = parser::context::parse(&content) {
                let result = {
                    let mut rng = rand::thread_rng();
                    format!("{} -> {}", &content, value.evalute(&mut rng))
                };
                let map = json!({
                    "content": result,
                    "message_reference": {
                        "message_id": *msg.id.as_u64()
                    }
                });
                let _ = context.http.send_message(msg.channel_id.0, &map).await;
            } else {
                let map = json!({
                    "content": format!("It works! your message is: {}", &content),
                    "message_reference": {
                        "message_id": *msg.id.as_u64()
                    }
                });

                let _ = context.http.send_message(msg.channel_id.0, &map).await;
            }
        }
    }
}
