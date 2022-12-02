/// Commands that serves testing purposes of the library or bot's capability
use crate::prelude::*;
use build_timestamp::build_time;

#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    build_time!("%Y%m%d %H:%M:%S UTC%z");
    ctx.say(format!("```version = 0.0.7\nlast-update ~= 20221201 23:30:00 UTC+0800\nbuild-time = {BUILD_TIME}```"))
        .await?;
    Ok(())
}

/// Test command that require a string input
#[poise::command(slash_command)]
pub async fn test_input(ctx: Context<'_>) -> Result<(), Error> {
    let number_a = rand::thread_rng().gen_range(1000..10000);
    let number_b = rand::thread_rng().gen_range(1000..10000);
    let reply = ctx
        .send(|msg| {
            msg.content(format!("{} + {} = ?", number_a, number_b))
                .components(|comp| {
                    comp.create_action_row(|row| {
                        row.create_button(|btn| {
                            btn.custom_id("test_input.submit_btn")
                                .style(serenity::ButtonStyle::Primary)
                                .label("submit an answer")
                        })
                    })
                })
        })
        .await?;

    let mut msg = reply.into_message().await?;
    let react = match msg
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(10))
        .await
    {
        Some(react) => react,
        None => {
            msg.edit(&ctx, |m| m.components(|c| c)).await?;
            return Ok(());
        }
    };
    msg.edit(&ctx, |m| m.components(|c| c)).await?;
    if react.user.id == ctx.author().id {
        react
            .create_interaction_response(&ctx, |r| {
                r.kind(serenity::InteractionResponseType::Modal)
                    .interaction_response_data(|d| {
                        d.custom_id("test_input.modal")
                            .title(format!("{number_a}+{number_b} = ?"))
                            .components(|component| {
                                component.create_action_row(|ar| {
                                    ar.create_input_text(|it| {
                                        it.style(serenity::InputTextStyle::Short)
                                            .required(true)
                                            .custom_id("test_input.modal.answer")
                                            .label("answer")
                                    })
                                })
                            })
                    })
            })
            .await?;

        match msg
            .await_modal_interaction(&ctx)
            .timeout(Duration::from_secs(10))
            .await
        {
            Some(react) => {
                if react.user.id == ctx.author().id {
                    if let serenity::ActionRowComponent::InputText(text) =
                        &react.data.components[0].components[0]
                    {
                        if let Ok(answer) = text.value.trim().parse::<u32>() {
                            if answer == number_a + number_b {
                                react
                                    .create_interaction_response(&ctx, |response| {
                                        response
                                    .kind(
                                        serenity::InteractionResponseType::ChannelMessageWithSource,
                                    )
                                    .interaction_response_data(|msg| msg.content("Nice"))
                                    })
                                    .await?;
                                Ok(())
                            } else {
                                react
                                    .create_interaction_response(&ctx, |response| {
                                        response
                                    .kind(
                                        serenity::InteractionResponseType::ChannelMessageWithSource,
                                    )
                                    .interaction_response_data(|msg| msg.content("Stoopid"))
                                    })
                                    .await?;
                                Ok(())
                            }
                        } else {
                            react
                                .create_interaction_response(&ctx, |response| {
                                    response
                                    .kind(
                                        serenity::InteractionResponseType::ChannelMessageWithSource,
                                    )
                                    .interaction_response_data(|msg| {
                                        msg.content("Can't you type number properly?")
                                    })
                                })
                                .await?;
                            Ok(())
                        }
                    } else {
                        unreachable!();
                    }
                } else {
                    react
                        .create_interaction_response(&ctx, |response| {
                            response
                                .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|msg| msg.content("No cheating!"))
                        })
                        .await?;
                    Ok(())
                }
            }
            None => {
                react
                    .create_followup_message(&ctx, |msg| msg.content("You're thinking too slow"))
                    .await?;
                Ok(())
            }
        }
    } else {
        react
            .create_interaction_response(&ctx, |response| {
                response
                    .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| msg.content("Type your own command!"))
            })
            .await?;
        Ok(())
    }
}

/// Test function that require authentication
#[poise::command(slash_command)]
pub async fn test_auth(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|msg| {
        msg.content("Click to authenticate").components(|comp| {
            comp.create_action_row(|row| {
                row.create_button(|btn| {
                    btn.custom_id("test_auth.auth_btn")
                        .style(serenity::ButtonStyle::Primary)
                        .label("Authenticate!")
                })
            })
        })
    })
    .await?;
    Ok(())
}

pub async fn test_auth_btn_handler<'a>(
    ctx: &serenity::Context,
    interaction: &ResponsibleInteraction<'a>,
) -> Result<(), Error> {
    if auth::authenticate(ctx, &interaction, "test_auth").await? {
        interaction
            .create_followup_message(&ctx, |msg| msg.content("(ᗜˬᗜ)"))
            .await?;
    } else {
        interaction
            .create_followup_message(&ctx, |msg| msg.content("(ᗜ˰ᗜ)"))
            .await?;
    }
    return Ok(());
}
