use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
    channel::Message
};
use serenity::framework::standard::{
    Args,
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};
use rand::seq::SliceRandom; 
use std::env;
use std::collections::HashMap;

extern crate dotenv;

#[group]
#[commands(ping, quote, join_team)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!")) // set the bot's prefix to "!"
        .group(&GENERAL_GROUP);
    // Use dotenv crate to get environment variables from .env file. 
    dotenv::dotenv().expect("Error reading .env file. Is the file present?");
    let token = env::var("DISCORD_TOKEN")
        .expect("Error reading 'DISCORD_TOKEN', is variable present in .env file?");
    
    // Login with a bot token from the environment
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

	msg.reply(ctx, quote_reply).await?;

	Ok(())
}

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;
    Ok(())
}

#[command]
pub async fn join_team(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    // This function needs to enable people to join competition teams. 
    // Usage: !join_team <ccdc, cptc, cyberforce, hivestorm, mdc3, cyberrange>
    //
    // Will need to take in message after command and iterate through it, breaking at whitespace
    // and adding people to roles for each team they want to join.

    // This gets all arguments that were passed with the command. 
    let assigned_roles = args.rest();
    // We create a hashmap (like a dictionary in python) to store all the possible roles 
    // and store the full names of the role. 
    // 
    // Key = what the user inputs, you'll want this to be one word as we split by whitespace.
    // Value = the full name of the role. 
    let possible_roles_hash: HashMap<&str, &str> = 
        [("ccdc", "CCDC Team Member"), 
        ("cptc", "CPTC Team Member"), 
        ("cyberforce", "Cyberforce Team Member"),
        ("mdc3", "MDC3 Team Member"),
        ("hivestorm", "Hivestorm Team Member"),
        ("cyberrange", "Cyber-range Member"),
        ("bot-dev", "Bot Developer")]
        .iter().cloned().collect();
    // FIXME loop through the words and check and see if it contains the keys from the hashmap,
    // if it does add the value of the hashmap to a queue. Then pop items off the queue to add those roles. 
    for role in assigned_roles.split_whitespace() {
        if possible_roles_hash.contains_key(&role) {
            if let Some(guild) = msg.guild(&ctx.cache).await {
                // We have to check and see if the guild has the role available. So we call the function
                // `role_by_name()` function on the guild object. 
                // `role_by_name()` returns a role ID which we can use to add the role to the user.  
                let certain_role = possible_roles_hash.get(role);
                if let Some(new_role) = guild.role_by_name(certain_role.unwrap()) {
                    // If there ISN'T a role we need to expect an error to be returned, if not we say
                    // something in the channel!
                    if let Err(why) = msg.channel_id.say(&ctx.http, &format!("Assigning role: {}", role)).await
                    {
                        // If we run into an error sending the message we print the error message. 
                        // (This will NOT print to the Discord Server, but to the server where the bot is)
                        println!{"Something went wrong: {:?}", why};
                    }
                    // Next we get the member information which we can also get from the message object.
                    // We make it mutable to ensure we can change the variable (since we're assigning roles!) 
                    let mut mem = msg.member(ctx).await.unwrap();
                    // Again we're doing something that might return an error, so we expect it.
                    // We're attempting to add the role to our mutable member object. 
                    if let Err(why) = mem.add_role(ctx, new_role).await {
                        //Print an error message if the role change fails. 
                        println!{"The role was not changed. Reason: {:?}", why};
                    }
                }
            }    
        } else {
        // If we can't find the roles in the list we need to tell somebody!
        msg.channel_id
            .say(&ctx.http, format!("Could not find role for: {}", role))
            .await?;

        }
    } 
    // Let them know we completed successfully.
    return Ok(())
}
