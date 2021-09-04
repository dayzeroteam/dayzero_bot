use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use rand::seq::SliceRandom; 
use std::env;

#[group]
#[commands(ping, quote)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
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
pub async fn quote(ctx: &Context, msg: &Message) -> CommandResult {
	let quotes = vec![
					"Biscuit: They have a party for two days and then get to meet the kids they fucked",
					"Biscuit: There's always a furry in the backend",
					"Creel: They replaced my bash shell with nyan cat",
					"MacDaddy: I was using Linux while you were still doodooing in diapers!",
					"MacDaddy: How hard is it to pull out a pistol and shoot someone?",
					"Dylan: I have to turn my security onion off during the summer - too hot",
					"Tristen: He's got an insurance policy - these hands",
					"Tristen: All the girls were like sploosh",
					"Tristen: According to company policy, we don't negotiate with cyberterrorists",
					"Thanh: I'm a nice guy, I didn't pee on him",
					"Chris: Category? MILF.",
					"Chris: That trophy has nice birthing hips.",
					"Nathan: Weird DeMarcus, I thought you were circumcised!",
					"Nathan: How about I not shit on the floor?",
					"Ryan: Does anyone want my English degree? I have no use for it."
    ];
    
    let quote_reply = quotes.choose(&mut rand::thread_rng()).unwrap();

	msg.reply(ctx, quote_reply);

	Ok(())
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
