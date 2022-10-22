use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn reply_error(ctx: &Context, cmd: &ApplicationCommandInteraction, msg: &str) {
    let mut embed = CreateEmbed::default();
    embed
        .author(|author| { author.icon_url(get_self_pfp_link(ctx)).name("There was an error") })
        .description(msg)
        .color(Colour::RED);

    cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.add_embed(embed))
    }).await.expect("Could not send embed");
}

pub fn get_self_pfp_link(ctx: &Context) -> String {
    match ctx.cache.current_user().avatar_url() {
        Some(avatar) => avatar,
        None => ctx.cache.current_user().default_avatar_url(),
    }
}