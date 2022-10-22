use serenity::client::Context;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::utils::Colour;

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
            command_utils::reply_error(ctx, cmd, "Could not parse response.").await;
            return;
        }
    };

    let year = json["year"].as_str().unwrap().to_owned();

    let mut month = json["month"].as_str().unwrap().to_owned();
    if month.len() == 1 {
        month = format!("0{}", month);
    }

    let mut day = json["day"].as_str().unwrap().to_owned();
    if day.len() == 1 {
        day = format!("0{}", day);
    }

    let mut embed = CreateEmbed::default();
    embed
        .author(|author| {
            author
                .url(format!("https://xkcd.com/{}/", &json["num"].as_i64().unwrap()))
                .icon_url(command_utils::get_self_pfp_link(ctx))
                .name(&json["title"].as_str().unwrap())
        })
        .image(&json["img"].as_str().unwrap())
        .description(&json["alt"].as_str().unwrap())
        .footer(|footer| footer.text(format!("Query: {}", &query)))
        .timestamp(format!("{}-{}-{}T13:15:30Z", year, month, day))
        .color(Colour::ORANGE);

    cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| data.add_embed(embed))
    }).await.expect("Could not send embed");
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("xkcd")
        .description("Sends an xkcd comic in chat")
        .create_option(|option| {
            option
                .name("query")
                .description("The comic's number or name")
                .kind(CommandOptionType::String)
        })
}