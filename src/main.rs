use std::fs;
use std::fs::File;
use std::path::Path;

use serenity::async_trait;
use serenity::Client;
use serenity::client::{EventHandler, Context};
use serenity::prelude::GatewayIntents;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::Command;
use serenity::model::prelude::interaction::Interaction;
use serenity::model::prelude::Activity;
use serenity::model::user::OnlineStatus;

mod xkcd_utils;
mod commands;
mod command_utils;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected successfully as {}!", ready.user.name);

        let _commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::xkcd_command::register(command))
                .create_application_command(|command| commands::bxkcd_command::register(command))
        }).await;

        ctx.set_presence(Some(Activity::playing("xkcd")), OnlineStatus::Idle).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = &interaction {
            match command.data.name.as_str() {
                "xkcd" => commands::xkcd_command::run(&command, &ctx).await,
                "bxkcd" => commands::bxkcd_command::run(&command, &ctx).await,
                _ => panic!("Tried to access unknown command"),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting up xkcd bot...");

    let token = get_token();

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler).await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

fn get_token() -> String {
    let file_exists = Path::new("token.txt").exists();

    if !file_exists {
        let _file_create_result = File::create("token.txt");

        panic!(
            "Could not read token from token.txt file. Please ensure the file exists and that it contains your bot token."
        );
    }

    let result = fs::read_to_string("token.txt");
    match result {
        Ok(contents) => contents,
        Err(error) => panic!("Could not read token from token.txt file. Error: {:?}", error),
    }
}