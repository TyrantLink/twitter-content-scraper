static CHANNEL_ID:u64 = 6969696969696969420; // Discord channel id. bot requires read, send, and embed permissions
static DISCORD_TOKEN:&str = "absolutely not."; // discord bot token.

use std::env;
use std::{thread::sleep, time::Duration};
use std::process::exit;

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::channel::Message;
use serenity::model::channel::Channel;
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, _ready: Ready) {
		let channel: Channel = ctx.http.get_channel(CHANNEL_ID).await.unwrap();
		let url: String = env::args().last().unwrap();
		let http= ctx.http.clone();
		channel.id().say(http,url).await.unwrap();
	}
	async fn message(&self, ctx: Context, msg: Message) {
		if msg.content.contains("twitter.com") {
			loop {
				sleep(Duration::from_secs(1));
				let http = ctx.http.clone();
				let msg: Message = msg.channel_id.message(http,msg.id).await.unwrap();
				if !msg.embeds.last().is_none() {
					println!("{}",msg.embeds.last().unwrap().description.clone().unwrap());
					msg.delete(ctx.http).await.unwrap();
					exit(0)}
			}
		}
	}	
}
#[tokio::main]
async fn main() {
	if !env::args().last().unwrap().contains("twitter.com") {
		println!("you must provide a twitter url as an argument");
		exit(0)}
	let intents = GatewayIntents::GUILD_MESSAGES|GatewayIntents::MESSAGE_CONTENT;
	let mut client = Client::builder(DISCORD_TOKEN, intents).event_handler(Handler).await.expect("Err creating client");
	if let Err(why) = client.start().await {
		println!("Client error: {:?}", why);
	}
}