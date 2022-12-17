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
    let interaction = match ctx {
        Application(application) => match application.interaction {
            poise::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                interaction
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let count = count.unwrap_or(1);
    let auth_result = auth::authenticate(
        ctx.serenity_context(),
        &ResponsibleInteraction::ApplicationCommand(interaction),
        "ask",
    )
    .await?;
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
        .create_followup_message(ctx, |message| {
            message.ephemeral(true).content(format!(
                "*Message from the mysterious stdin:*\n\n{}",
                stdin_response
            ))
        })
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
    let interaction = match ctx {
        Application(application) => match application.interaction {
            poise::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                interaction
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let auth_result = auth::authenticate(
        ctx.serenity_context(),
        &ResponsibleInteraction::ApplicationCommand(interaction),
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
        .create_followup_message(ctx, |message| message.ephemeral(true).content(response))
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
    let interaction = match ctx {
        Application(application) => match application.interaction {
            poise::ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                interaction
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };
    let count = count.unwrap_or(1);
    let auth_result = auth::authenticate(
        ctx.serenity_context(),
        &ResponsibleInteraction::ApplicationCommand(interaction),
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
                    .create_followup_message(ctx, |message| {
                        message
                            .ephemeral(true)
                            .content(format!("Getting message(s) from stdin...{}", i + 1))
                    })
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
            .create_followup_message(ctx, |message| message.ephemeral(true).content(&response))
            .await?;
    }
    Ok(())
}

/// Displays information about the bot
#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    use build_time::build_time_local;

    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    ctx.send(|msg| {
        msg.ephemeral(true).content(format!(
            "```version = {}\nbuild-time = {}```",
            VERSION.unwrap_or("UNKNOWN"),
            build_time_local!("%Y-%m-%d %H:%M:%S %:z")
        ))
    })
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
