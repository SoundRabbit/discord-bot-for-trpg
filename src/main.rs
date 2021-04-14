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
    comment_pattern: Regex,
}

impl Default for Handler {
    fn default() -> Self {
        Self {
            mention_pattern: Regex::new(r"<@!?\d+>").unwrap(),
            comment_pattern: Regex::new(r"//.*$").unwrap(),
        }
    }
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, msg: Message) {
        if msg.mentions_me(&context).await.unwrap_or(false) {
            // メンションを削除
            println!("{}", &msg.content);
            let content = self.mention_pattern.replace_all(&msg.content, "");
            let content = self.comment_pattern.replace_all(&content, "");
            println!("{}", &content);
            match parser::context::parse(&content) {
                Ok(exp0) => {
                    let result = {
                        let mut env = runtime::Environment::new();
                        async_std::task::block_on(runtime::built_in_function::set_default(
                            &mut env,
                        ));
                        let mut rng = rand::thread_rng();
                        let mut log = vec![];
                        let evaluted = exp0.evalute(&mut env, &mut rng, &mut log);
                        let mut res = format!("{}\n", &content);
                        for a_line in log {
                            res += format!(" -> {}", a_line).as_str();
                        }
                        res += format!(" -> {}", evaluted).as_str();
                        let debug_msg = include_str!("./msg");
                        if debug_msg.len() > 0 {
                            res += "\n";
                            res += debug_msg;
                        };
                        res
                    };
                    let map = json!({
                        "content": result,
                        "message_reference": {
                            "message_id": *msg.id.as_u64()
                        }
                    });
                    let _ = context.http.send_message(msg.channel_id.0, &map).await;
                }
                Err(err) => {
                    let map = json!({
                        "content": format!("{:?}", err),
                        "message_reference": {
                            "message_id": *msg.id.as_u64()
                        }
                    });

                    let _ = context.http.send_message(msg.channel_id.0, &map).await;
                }
            }
        }
    }
}
