/// Commands that serves testing purposes of the library or bot's capability
use crate::prelude::*;
use poise::futures_util::StreamExt;

/// Ask stdin something and wait for response
#[poise::command(slash_command)]
pub async fn ask(
    ctx: Context<'_>,
    #[description = "What do you want do ask the mysterious stdin?"]
    #[rest]
    something: Option<String>,
    #[description = "How many lines do you want \
                        the mysterious stdin to answer?"]
    count: Option<u32>,
) -> Result<(), Error> {
    let interaction = slash_ctx_as_responsibe_interaction(&ctx);
    let count = count.unwrap_or(1);
    let auth_result = auth::authenticate(ctx.serenity_context(), &interaction, "ask").await?;
    let stdin_response = if auth_result {
        let mut stdin_response = String::new();
        let stdio_lock = tokio::time::timeout(
            tokio::time::Duration::from_secs(300),
            ctx.data().stdio_lock.lock(),
        )
        .await;
        if stdio_lock.is_err() {
            "*The mysterious stdin is busy talking to someone else...*".to_string()
        } else {
            let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
            println!(
                "{}",
                something
                    .as_ref()
                    .map_or_else(|| "Something?", |s| s.as_str())
            );
            stdin_response.push_str(&loop {
                let line = stdin_reader.next().await.transpose()?;
                if !line.is_none() {
                    break line.unwrap();
                }
            });
            stdin_response.push('\n');
            for _ in 1..count {
                println!("More?");
                stdin_response.push_str(&loop {
                    let line = stdin_reader.next().await.transpose()?;
                    if !line.is_none() {
                        break line.unwrap();
                    }
                });
                stdin_response.push('\n');
            }
            drop(stdin_reader);
            drop(stdio_lock);
            stdin_response
        }
    } else {
        "*The mysterious stdin didn't answer...*".to_string()
    };
    interaction
        .create_followup(
            ctx,
            serenity::CreateInteractionResponseFollowup::new()
                .ephemeral(true)
                .content(format!(
                    "*Message from the mysterious stdin:*\n\n{}",
                    stdin_response
                )),
        )
        .await?;
    Ok(())
}

/// Tell stdin something
#[poise::command(slash_command)]
pub async fn msg(
    ctx: Context<'_>,
    #[description = "What do you want to say to the mysterious stdin?"]
    #[rest]
    something: String,
) -> Result<(), Error> {
    let interaction = slash_ctx_as_responsibe_interaction(&ctx);
    let auth_result = auth::authenticate(
        ctx.serenity_context(),
        &interaction,
        format!("msg_{}", &something).as_str(),
    )
    .await?;
    let response = if auth_result {
        let stdio_lock = tokio::time::timeout(
            tokio::time::Duration::from_secs(300),
            ctx.data().stdio_lock.lock(),
        )
        .await;
        if stdio_lock.is_err() {
            "*The mysterious stdin is busy talking to someone else...*"
        } else {
            println!("{}", something);
            drop(stdio_lock);
            "*The mysterious stdin: ...*"
        }
    } else {
        "*The mysterous stdin refused to listen to you...*"
    };
    interaction
        .create_followup(
            ctx,
            serenity::CreateInteractionResponseFollowup::new()
                .ephemeral(true)
                .content(response),
        )
        .await?;
    Ok(())
}

/// Get some words from stdin
#[poise::command(slash_command)]
pub async fn getline(
    ctx: Context<'_>,
    #[description = "How many lines do you want \
                        from the mysterious stdin?"]
    count: Option<u32>,
) -> Result<(), Error> {
    let interaction = slash_ctx_as_responsibe_interaction(&ctx);
    let count = count.unwrap_or(1);
    let auth_result = auth::authenticate(
        ctx.serenity_context(),
        &interaction,
        format!("getline_{}", count).as_str(),
    )
    .await?;
    if auth_result {
        let stdio_lock = tokio::time::timeout(
            tokio::time::Duration::from_secs(300),
            ctx.data().stdio_lock.lock(),
        )
        .await;
        let response = if stdio_lock.is_err() {
            "*The mysterious stdin is busy talking to someone else...*".to_string()
        } else {
            let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
            let mut stdin_response = String::new();
            for i in 0..count {
                interaction
                    .create_followup(
                        ctx,
                        serenity::CreateInteractionResponseFollowup::new()
                            .ephemeral(true)
                            .content(format!("Getting message(s) from stdin...{}", i + 1)),
                    )
                    .await?;

                stdin_response.push_str(&loop {
                    let line = stdin_reader.next().await.transpose()?;
                    if !line.is_none() {
                        break line.unwrap();
                    }
                });
                stdin_response.push('\n');
            }
            drop(stdin_reader);
            drop(stdio_lock);
            stdin_response
        };
        interaction
            .create_followup(
                ctx,
                serenity::CreateInteractionResponseFollowup::new()
                    .ephemeral(true)
                    .content(&response),
            )
            .await?;
    }
    Ok(())
}

/// Test command that require a string input
#[poise::command(slash_command)]
pub async fn test_input(ctx: Context<'_>) -> Result<(), Error> {
    let number_a = rand::thread_rng().gen_range(1000..10000);
    let number_b = rand::thread_rng().gen_range(1000..10000);
    let test_input_component = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("test_input.submit_btn")
            .style(serenity::ButtonStyle::Primary)
            .label("submit an answer"),
    ])];
    let reply = ctx
        .send(
            poise::CreateReply::default()
                .content(format!("{} + {} = ?", number_a, number_b))
                .components(test_input_component),
        )
        .await?;

    let mut msg = reply.into_message().await?;
    let react = match msg
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(10))
        .await
    {
        Some(react) => react,
        None => {
            msg.edit(&ctx, serenity::EditMessage::new().components(vec![]))
                .await?;
            return Ok(());
        }
    };
    msg.edit(&ctx, serenity::EditMessage::new().components(vec![]))
        .await?;
    let test_input_modal_component = vec![serenity::CreateActionRow::InputText(
        serenity::CreateInputText::new(
            serenity::InputTextStyle::Short,
            "test_input.modal.answer",
            "answer",
        )
        .required(true),
    )];
    if react.user.id == ctx.author().id {
        react
            .create_response(
                &ctx,
                serenity::CreateInteractionResponse::Modal(
                    serenity::CreateModal::new(
                        "test_input.modal",
                        format!("{number_a}+{number_b} = ?"),
                    )
                    .components(test_input_modal_component),
                ),
            )
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
                        if let Ok(answer) = text
                            .value
                            .as_ref()
                            .unwrap_or(&String::default())
                            .trim()
                            .parse::<u32>()
                        {
                            if answer == number_a + number_b {
                                react
                                    .create_response(
                                        &ctx,
                                        serenity::CreateInteractionResponse::Message(
                                            serenity::CreateInteractionResponseMessage::new()
                                                .content("Nice"),
                                        ),
                                    )
                                    .await?;
                                Ok(())
                            } else {
                                react
                                    .create_response(
                                        &ctx,
                                        serenity::CreateInteractionResponse::Message(
                                            serenity::CreateInteractionResponseMessage::new()
                                                .content("Stoopid"),
                                        ),
                                    )
                                    .await?;
                                Ok(())
                            }
                        } else {
                            react
                                .create_response(
                                    &ctx,
                                    serenity::CreateInteractionResponse::Message(
                                        serenity::CreateInteractionResponseMessage::new()
                                            .content("Can't you type number properly?"),
                                    ),
                                )
                                .await?;
                            Ok(())
                        }
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
                    Ok(())
                }
            }
            None => {
                react
                    .create_followup(
                        &ctx,
                        serenity::CreateInteractionResponseFollowup::new()
                            .content("You're thinking too slow"),
                    )
                    .await?;
                Ok(())
            }
        }
    } else {
        react
            .create_response(
                &ctx,
                serenity::CreateInteractionResponse::Message(
                    serenity::CreateInteractionResponseMessage::new()
                        .content("Type your own command!"),
                ),
            )
            .await?;
        Ok(())
    }
}

/// Test function that require authentication
#[poise::command(slash_command)]
pub async fn test_auth(ctx: Context<'_>) -> Result<(), Error> {
    let auth_test_components = vec![serenity::CreateActionRow::Buttons(vec![
        serenity::CreateButton::new("test_auth.auth_btn")
            .style(serenity::ButtonStyle::Primary)
            .label("Authenticate!"),
    ])];
    ctx.send(
        poise::CreateReply::default()
            .content("Click to authenticate")
            .components(auth_test_components),
    )
    .await?;
    Ok(())
}

pub async fn test_auth_btn_handler<'a>(
    ctx: &serenity::Context,
    interaction: &ResponsibleInteraction<'a>,
) -> Result<(), Error> {
    if auth::authenticate(ctx, &interaction, "test_auth").await? {
        interaction
            .create_followup(
                &ctx,
                serenity::CreateInteractionResponseFollowup::new().content("(ᗜˬᗜ)"),
            )
            .await?;
    } else {
        interaction
            .create_followup(
                &ctx,
                serenity::CreateInteractionResponseFollowup::new().content("(ᗜ˰ᗜ)"),
            )
            .await?;
    }
    return Ok(());
}
