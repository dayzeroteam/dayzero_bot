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
    // Usage: !join_team <ccdc, cptc, cyberforce, hivestorm, mdc3, cyber-range>
    //
    // Will need to take in message after command and iterate through it, breaking at whitespace
    // and adding people to roles for each team they want to join.
    //

    let possible_role = args.rest();
    
    // We have to get the "guild" or server, we can get this information from the message object.
    if let Some(guild) = msg.guild(&ctx.cache).await {
        // We have to check and see if the guild has the role available. So we call the function
        // `role_by_name()` function on the guild object. 
        // `role_by_name()` returns a role ID which we can use to add the role to the user.  
        if let Some(role) = guild.role_by_name(possible_role) {
            // If there ISN'T a role we need to expect an error to be returned, if not we say
            // something in the channel!
            if let Err(why) = msg.channel_id.say(&ctx.http, &format!("Assigning role: {}", possible_role)).await
            {
                // If we run into an error sending the message we print the error message. 
                // (This will NOT print to the Discord Server, but to the server where the bot is)
                println!{"AH CRAP ERROR: {:?}", why};
            }
            // Next we get the member information which we can also get from the message object.
            // We make it mutable to ensure we can change the variable (since we're assigning roles!) 
            let mut mem = msg.member(ctx).await.unwrap();
            // Again we're doing something that might return an error, so we expect it.
            // We're attempting to add the role to our mutable member object. 
            if let Err(why) = mem.add_role(ctx, role).await {
                //Print an error message if the role change fails. 
                println!{"AH CRAP ROLE NOT CHANGED: {:?}", why};
            }
        // If we're succesful get here and exit.            
        return Ok(())
        }

    }    

    // If there is no role we send a message letting them know!
    msg.channel_id
        .say(&ctx.http, format!("Could not find role named: {:?}", possible_role))
        .await?;

    Ok(())
}
