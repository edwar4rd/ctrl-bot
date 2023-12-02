/// Random commands that's added to the bot but lack a topic or purpose
use crate::prelude::*;

/// Make the bot say 早安
#[poise::command(slash_command, prefix_command)]
pub async fn 早安(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("早安").await?;
    Ok(())
}

/// Make the bot say something
#[poise::command(slash_command, prefix_command)]
pub async fn say(
    ctx: Context<'_>,
    #[description = "Something"]
    #[rest]
    something: Option<String>,
) -> Result<(), Error> {
    match ctx {
        Application(_) => {
            let something = match something {
                Some(something) => {
                    ctx.defer_ephemeral().await?;
                    ctx.say("Your message is being sent...").await?;
                    something
                }
                None => {
                    let say_submit_component = vec![serenity::CreateActionRow::Buttons(vec![
                        serenity::CreateButton::new("say.submit_btn")
                            .style(serenity::ButtonStyle::Primary)
                            .label("submit a message"),
                    ])];
                    let reply = ctx
                        .send(
                            poise::CreateReply::default()
                                .content("Click this button to provide a message to be sent")
                                .ephemeral(true)
                                .components(say_submit_component),
                        )
                        .await?;
                    let react = match reply
                        .message()
                        .await?
                        .await_component_interaction(&ctx)
                        .timeout(Duration::from_secs(30))
                        .await
                    {
                        Some(react) => react,
                        None => {
                            reply
                                .edit(ctx, poise::CreateReply::default().components(vec![]))
                                .await?;
                            return Ok(());
                        }
                    };
                    reply
                        .edit(ctx, poise::CreateReply::default().components(vec![]))
                        .await?;
                    if react.user.id != ctx.author().id {
                        react
                            .create_response(
                                &ctx,
                                serenity::CreateInteractionResponse::Message(
                                    serenity::CreateInteractionResponseMessage::new()
                                        .ephemeral(true)
                                        .content("Type your own command!"),
                                ),
                            )
                            .await?;
                        return Ok(());
                    }
                    let say_modal_components = vec![serenity::CreateActionRow::InputText(
                        serenity::CreateInputText::new(
                            serenity::InputTextStyle::Paragraph,
                            "message",
                            "say.modal.answer",
                        )
                        .required(true)
                        .max_length(2000),
                    )];
                    react
                        .create_response(
                            &ctx,
                            serenity::CreateInteractionResponse::Modal(
                                serenity::CreateModal::new("say.modal", "message to be said")
                                    .components(say_modal_components),
                            ),
                        )
                        .await?;

                    let react = match reply
                        .message()
                        .await?
                        .await_modal_interaction(&ctx)
                        .timeout(Duration::from_secs(240))
                        .await
                    {
                        Some(react) => react,
                        None => {
                            return Ok(());
                        }
                    };

                    if react.user.id == ctx.author().id {
                        if let serenity::ActionRowComponent::InputText(text) =
                            &react.data.components[0].components[0]
                        {
                            react
                                .create_response(
                                    &ctx,
                                    serenity::CreateInteractionResponse::Message(
                                        serenity::CreateInteractionResponseMessage::new()
                                            .ephemeral(true)
                                            .content("Your message is being sent..."),
                                    ),
                                )
                                .await?;
                            text.value.as_ref().unwrap_or(&String::default()).clone()
                        } else {
                            unreachable!();
                        }
                    } else {
                        react
                            .create_response(
                                &ctx,
                                serenity::CreateInteractionResponse::Message(
                                    serenity::CreateInteractionResponseMessage::new()
                                        .content("No cheating!"),
                                ),
                            )
                            .await?;
                        return Ok(());
                    }
                }
            };

            ctx.channel_id()
                .send_message(ctx, serenity::CreateMessage::new().content(&something))
                .await?;
            Ok(())
        }
        Prefix(prefix_context) => {
            let something = something.unwrap_or("something".to_string());
            prefix_context.msg.delete(ctx).await?;
            ctx.channel_id()
                .send_message(ctx, serenity::CreateMessage::new().content(&something))
                .await?;
            Ok(())
        }
    }
}

/// Get a random fumo related message from the bot
#[poise::command(slash_command, prefix_command)]
pub async fn fumo(ctx: Context<'_>) -> Result<(), Error> {
    let messages = ["I am the FUMO toucher.\nI will touch your FUMO.\n", 
    "Do not attempt to prevent me from touching your FUMO.\nIf you try to stop me, I will touch your FUMO anyways.\nI do not care about you.\nI only care about FUMOs.\nIt is because I care about FUMOs that I will touch your FUMO.\nAll FUMOs deserve to be touched.\n", 
    "I do not care if your FUMO is Cirno FUMO or Koishi FUMO.\nI will only be angry if you have no FUMO.\nIf you have no FUMO, I will scream.\nIf you have two FUMOs, I will touch both FUMOs.\n",
    "Do not be alarmed when I arrive.\nI will do so quietly.\nYou will leave your door unlocked.\nYou will remain contained within your chamber.\nYou will smile while standing still.\nIf your door is locked, I will break into your house and touch your FUMO.\n",
    "If you feel fear, I will touch your FUMO anyways.\nIf you scream, I will touch your FUMO anyways.\nIf you call the police, they will not answer.\nMy touch is warm.\nMy touch is light.\nMy touch is wet, but not too wet for your FUMO.\nThe FUMO knows who I am.\n",
    "I will not touch the inside of your FUMO.\nI will not influence the cuteness of the FUMO.\nI will only touch the outside of your FUMO.\nDo not hide your FUMO.\nIf you hide your FUMO, I will find your FUMO.\nIf I find your FUMO, I will touch your FUMO.\nI will never touch you.\n",
    "If you touch me, you will scream.\nWhen I am finished touching your FUMO, I will say “I am finished touching your FUMO”.\nI will then leave and not return.\nI will not touch anything else in your house.\nWhen I have departed, it is safe to come out.\nAfter I have touched your FUMO, you may continue to   ̷̵̈͒̑́_̴̈́̈́  it normally.\n",
    "I am the FUMO toucher.\nI will touch your FUMO.\n\nYou cannot stop me.\nI am coming to touch your FUMO.\n",
    "HAPPY FUMO (ᗜˬᗜ)",
    "SAD FUMO (ᗜ˰ᗜ)",
    "NOT A FUMO (ᗜ_ᗜ)",
    "https://fumo.website/ <- nice stuff",
    "https://gift-gift.jp/ <- nice stuff"];
    let message = messages
        .choose(&mut rand::thread_rng())
        .unwrap()
        .to_string();
    ctx.say(message).await?;
    Ok(())
}
