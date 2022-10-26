use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use crate::xkcd_utils;
use crate::command_utils;

pub async fn run(cmd: &ApplicationCommandInteraction, ctx: &Context) {
    let query: &str;

    if cmd.data.options.len() != 1 {
        query = "standards";
    } else {
        let value = match &cmd.data.options[0].value {
            Some(val) => val,
            None => {
                command_utils::reply_error(ctx, cmd, "There was an error while trying to parse the command.").await;
                return;
            }
        };

        query = value.as_str().unwrap().trim();
    }

    let url = if query.parse::<u32>().is_ok() {
        xkcd_utils::get_xkcd_api_url_from_int(&query.parse::<u32>().unwrap())
    } else {
        match xkcd_utils::get_xkcd_api_url_from_string(&query).await {
            Ok(url) => url,
            Err(_) => {
                command_utils::reply_error(ctx, cmd, &format!("Couldn't find any results for the query: *{}*", query)).await;
                return;
            }
        }
    };

    let json = match xkcd_utils::get_json_from_url(&url).await {
        Ok(json) => json,
        Err(_) => {
            command_utils::reply_error(ctx, cmd, "No comic was found for the provided query.").await;
            return;
        }
    };

    cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| data.content(&json["img"].as_str().unwrap()))
    }).await.expect("Could not send embed");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("bxkcd")
        .description("Same as /xkcd but much, much simpler (just the image)")
        .create_option(|option| {
            option
                .name("query")
                .description("The comic's number or name")
                .kind(CommandOptionType::String)
        })
}