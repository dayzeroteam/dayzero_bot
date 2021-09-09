use serenity::async_trait;
use serenity::http::Http;
use serenity::model::id::RoleId;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::{
    channel::Message,
    guild::Member,
    gateway::Ready
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
pub async fn join_team(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // This function needs to enable people to join competition teams. 
    // Usage: !join_team <ccdc, cptc, cyberforce, hivestorm, mdc3, cyber-range>
    //
    // Will need to take in message after command and iterate through it, breaking at whitespace
    // and adding people to roles for each team they want to join.
    //
    //msg.reply(ctx, args.single::<String>().unwrap()).await?;
    //msg.reply(ctx, msg.author.id).await?;

    let possible_role = args.rest();
    //let server_id = 758456684036751420;
    //let member_id = msg.author.id.0;
    //let http_id = ctx.http;
    
    //let member1 = ctx.http.get_member(server_id, member_id).await?;
    //let role = 858559192481398865;

    //let roles = http_id.get_guild_roles(server_id);
    //msg.reply(http_id, &format("{:#?}", roles));
    //member1.add_roles(http_id, &role);
    
    if let Some(guild) = msg.guild(&ctx.cache).await {
        if let Some(role) = guild.role_by_name(possible_role) {
            if let Err(why) = msg.channel_id.say(&ctx.http, &format!("Role: {}", role.id)).await
            {
                println!{"AH CRAP ERROR: {:?}", why};
            }
            let mut mem = msg.member(ctx).await.unwrap();
            if let _update = mem.add_role(ctx, role) {
                msg.channel_id.say(&ctx.http, &format!("Role changed!")).await?; 
            }
        return Ok(())
        }

    }    

    msg.channel_id
        .say(&ctx.http, format!("Could not find role named: {:?}", possible_role))
        .await?;
    //msg.author.dm(&ctx, |m| {
    //    m.content("Adding your roles");
    
    //    m
    //}).await?;

    //msg.author.add_roles(858559192481398865);

    Ok(())
}
