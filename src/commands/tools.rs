/// Commands that serves as tools that provide information or control of the host computer of the bot and the bot itself
use crate::prelude::*;

/// Prints system information output by neofetch
#[poise::command(slash_command)]
pub async fn neofetch(
    ctx: Context<'_>,
    // #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let interaction = match &ctx {
        Application(application_context) => match application_context.interaction {
            ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                interaction
            }
            _ => {
                unreachable!()
            }
        },
        _ => {
            unreachable!();
        }
    };

    let response = if auth::authenticate(
        &ctx.serenity_context(),
        &ResponsibleInteraction::ApplicationCommand(interaction),
        "neofetch",
    )
    .await?
    {
        format!(
            "```{}```",
            String::from_utf8_lossy(
                &Command::new("neofetch")
                    .arg("--stdout")
                    .output()
                    .expect("Failed to get system information")
                    .stdout,
            )
        )
    } else {
        "Nope, wrong passcode lol\n".to_string()
    };
    interaction
        .create_followup_message(&ctx, |msg| msg.ephemeral(true).content(response))
        .await?;
    Ok(())
}

/// Make the bot ping to somewhere several times
#[poise::command(slash_command)]
pub async fn ping(
    ctx: Context<'_>,
    #[description = "a target IP adderss"] address: String,
    #[description = "Stop after sending count ECHO_REQUEST packets.(<=50)"] count: Option<u8>,
) -> Result<(), Error> {
    let interaction = match &ctx {
        Application(application_context) => match application_context.interaction {
            ApplicationCommandOrAutocompleteInteraction::ApplicationCommand(interaction) => {
                interaction
            }
            _ => {
                unreachable!()
            }
        },
        _ => {
            unreachable!();
        }
    };

    let count: u8 = count.unwrap_or(4);

    if count > 50 {
        interaction
            .create_interaction_response(&ctx, |response| {
                response
                    .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|msg| {
                        msg.ephemeral(true)
                            .content("Given `count` is greater than 50!")
                    })
            })
            .await?;
        return Ok(());
    }

    let address = match address.parse::<IpAddr>() {
        Ok(address) => address,
        Err(_) => {
            interaction
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.ephemeral(true)
                                .content("Given `address` isn't valid Ipv4 address!")
                        })
                })
                .await?;
            return Ok(());
        }
    };

    let response = if auth::authenticate(
        &ctx.serenity_context(),
        &ResponsibleInteraction::ApplicationCommand(interaction),
        &format!("{}_{}", count, address.to_string()),
    )
    .await?
    {
        match &Command::new("ping")
            .arg("-i")
            .arg("0.2")
            .arg("-c")
            .arg(count.to_string())
            .arg(address.to_string())
            .output()
        {
            Ok(output) => {
                let response = String::from_utf8_lossy(&output.stdout).to_string();
                if response.len() > 1990 {
                    let mut lines = response.lines();
                    let mut current = String::new();
                    loop {
                        match lines.next() {
                            Some(line) => {
                                if current.len() + line.len() > 1990 {
                                    interaction
                                        .create_followup_message(&ctx, |msg| {
                                            msg.ephemeral(true)
                                                .content(format!("```{}```", current))
                                        })
                                        .await?;
                                    current.clear();
                                }
                                current.push('\n');
                                current.push_str(line);
                            }
                            None => {
                                break format!("```{}```", current);
                            }
                        }
                    }
                } else {
                    format!("```{}```", response)
                }
            }
            Err(_) => "Failed to perform ping command".to_string(),
        }
    } else {
        "Nope, wrong passcode lol\n".to_string()
    };
    interaction
        .create_followup_message(&ctx, |msg| msg.ephemeral(true).content(response))
        .await?;
    Ok(())
}
