use base64;
use poise::{
    serenity_prelude as serenity,
    Context::{Application, Prefix},
};
use rand::prelude::*;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey};
use std::{iter, net::IpAddr};
// use sha3::{Digest, Sha3_512};
use dc_bot::ResponsibleInteraction;
use poise::ApplicationCommandOrAutocompleteInteraction;
use std::process::Command;
use std::time::{Duration, *};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

async fn authenticate<'a>(
    ctx: &serenity::Context,
    interaction: ResponsibleInteraction<'a>,
    action_data: &str,
) -> Result<bool, Error> {
    let challenge_random = iter::repeat_with(|| thread_rng().gen::<u8>())
        .take(192)
        .collect::<Vec<u8>>();
    let challenge_encoded = format!(
        "{}_{}_{}\n",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        base64::encode(&challenge_random),
        action_data /*
                    base64::encode({
                        let mut hasher = Sha3_512::new();
                        hasher.update(action_data);
                        hasher.update(challenge_random);
                        hasher.finalize()
                    })*/
    );

    interaction.create_interaction_response(&ctx, |response| response
        .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|msg| msg
            .ephemeral(true)
            .content(format!("Please authenticate by signing this random message(including the newline) with your secret key:\n```{challenge_encoded}```\n\n```openssl rsautl -sign -inkey privkey.pem | openssl enc -base64```"))
            .components(|components| components
                .create_action_row(|row| row
                    .create_button(|btn| btn
                        .custom_id("authenticate.button")
                        .label("Submit signed")
                        .style(serenity::ButtonStyle::Primary)
                    )
                )
            ))
        ).await?;

    let btn_reply = interaction.get_interaction_response(&ctx).await?;

    let btn_reply_react = match btn_reply
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(60))
        .await
    {
        Some(react) => react,
        None => {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("(Too slow!)"))
                .await?;
            return Ok(false);
        }
    };

    btn_reply_react
        .create_interaction_response(&ctx, |response| {
            response
                .kind(serenity::InteractionResponseType::Modal)
                .interaction_response_data(|modal| {
                    modal
                        .custom_id("authenticate.modal")
                        .title("Submit signed message to authenticate")
                        .components(|component| {
                            component.create_action_row(|action_row| {
                                action_row.create_input_text(|input_text| {
                                    input_text
                                        .style(serenity::InputTextStyle::Paragraph)
                                        .required(true)
                                        .custom_id("authenticate.modal.signed")
                                        .label("signed message")
                                })
                            })
                        })
                })
        })
        .await?;

    let modal_reply_react = match btn_reply
        .await_modal_interaction(&ctx)
        .timeout(Duration::from_secs(60))
        .await
    {
        Some(react) => react,
        None => {
            btn_reply_react
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("(Too slow!)"))
                .await?;
            return Ok(false);
        }
    };

    if let serenity::ActionRowComponent::InputText(text) =
        &modal_reply_react.data.components[0].components[0]
    {
        if let Ok(signed) = base64::decode(
            &text
                .value
                .trim()
                .as_bytes()
                .into_iter()
                .filter(|c| !c.is_ascii_whitespace())
                .map(|x| *x)
                .collect::<Vec<u8>>(),
        ) {
            if rsa::RsaPublicKey::from_public_key_pem(
                "-----BEGIN PUBLIC KEY-----
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAo7/fmoTQhWboiCHpuGF3
DmAmyeTZEvaGvAKzUeabnds9iA0UCCm5kPRKK0kWGj/xpBJxzyCRzxUvKvPtY02/
s8DdY/EMBcJOPLvd+VbGJsrSVkQnD5SmexyRuioZ2byFSPUFYZ5sNQzdI08XP4J2
ttI6jiu61cIO5JvPfjTntO40/dmpyb8olf/6Nifc62NnV8JGEsnTrd3QdrjCo3vj
t73FEKCccAJfJQtLZo5AFFLZpjTcXvEd1BucHf15qO0cu05toV7l/HICpupm9G2e
Q92bn2KN5zondTJRHo+xrTWVGUx0KH8WX/XdbsC1l6BfB7KIwL9rhMVDBtmCSBAI
7N6KGxGbLo22d7kEMKTWrxtz2fDUVysVgooAvbeeYQGfsmLoLB4Dyi33vGQFjwMs
bIElmgKQRubiCzwZ3EblFLIiEREMUZBwZQrLK2u92d7CE63wWoJEKIFBUJQo+ZJv
zkPogJ6VP7kLZCb0cbxTSobJtfrWFbWCHoF1WbbLktTw9b6dfn4PYv8oTZnukVFr
fGoE3XHvnuRa+SAkf0GafMSU3k+htCqBqIAqLRcRxc8lr+9ejOExBobh8ElNu+os
UmGJIty1t2qxNhtizyengaVjbNK9loSD2yzxS1wPR3jF68ztKkgmcnxEPH2Iq1zy
Dy7uxt3qNoJykUCNUqlNBNUCAwEAAQ==
-----END PUBLIC KEY-----
",
            )
            .unwrap()
            .verify(
                PaddingScheme::new_pkcs1v15_sign_raw(),
                &challenge_encoded.as_bytes(),
                &signed,
            )
            .is_ok()
            {
                modal_reply_react
                    .create_interaction_response(&ctx, |response| {
                        response
                            .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|msg| {
                                msg.ephemeral(true).content("(Authentication Succeeded!)")
                            })
                    })
                    .await?;
                Ok(true)
            } else {
                modal_reply_react
                    .create_interaction_response(&ctx, |response| {
                        response
                            .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|msg| {
                                msg.ephemeral(true).content("(Authentication Failed!)")
                            })
                    })
                    .await?;
                Ok(false)
            }
        } else {
            modal_reply_react
                .create_interaction_response(&ctx, |response| {
                    response
                        .kind(serenity::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.ephemeral(true)
                                .content("(Failed to parse responsed message)")
                        })
                })
                .await?;
            Ok(false)
        }
    } else {
        unreachable!();
    }
}

async fn handler(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::InteractionCreate { interaction } => match interaction.kind() {
            serenity::InteractionType::MessageComponent => {
                let interaction = interaction.clone().message_component().unwrap();
                match interaction.data.component_type {
                    serenity::ComponentType::Button => {
                        if interaction.data.custom_id == "test_auth.auth_btn" {
                            if authenticate(
                                ctx,
                                ResponsibleInteraction::MessageComponent(&interaction),
                                "test_auth",
                            )
                            .await?
                            {
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
                    }
                    _ => {}
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("version = 0.0.6\nlast-update ~= 20221201 11:40 UTC+8")
        .await?;
    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

/// Make the bot say something
#[poise::command(slash_command, prefix_command)]
async fn say(
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
                    let reply = ctx
                        .send(|msg| {
                            msg.content("Click this button to provide a message to be sent")
                                .ephemeral(true)
                                .components(|comp| {
                                    comp.create_action_row(|row| {
                                        row.create_button(|btn| {
                                            btn.custom_id("say.submit_btn")
                                                .style(serenity::ButtonStyle::Primary)
                                                .label("submit a message")
                                        })
                                    })
                                })
                        })
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
                            reply.edit(ctx, |m| m.components(|c| c)).await?;
                            return Ok(());
                        }
                    };
                    reply.edit(ctx, |m| m.components(|c| c)).await?;
                    if react.user.id != ctx.author().id {
                        react
                            .create_interaction_response(&ctx, |response| {
                                response
                                    .kind(
                                        serenity::InteractionResponseType::ChannelMessageWithSource,
                                    )
                                    .interaction_response_data(|msg| {
                                        msg.ephemeral(true).content("Type your own command!")
                                    })
                            })
                            .await?;
                        return Ok(());
                    }
                    react
                        .create_interaction_response(&ctx, |response| {
                            response
                                .kind(serenity::InteractionResponseType::Modal)
                                .interaction_response_data(|d| {
                                    d.custom_id("say.modal")
                                        .title("message to be said")
                                        .components(|component| {
                                            component.create_action_row(|ar| {
                                                ar.create_input_text(|it| {
                                                    it.style(serenity::InputTextStyle::Paragraph)
                                                        .required(true)
                                                        .custom_id("say.modal.answer")
                                                        .label("message")
                                                })
                                            })
                                        })
                                })
                        })
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
                                    .create_interaction_response(&ctx, |response| {
                                        response
                                    .kind(
                                        serenity::InteractionResponseType::ChannelMessageWithSource,
                                    )
                                    .interaction_response_data(|msg| msg.ephemeral(true).content("Your message is being sent..."))
                                    })
                                    .await?;
                            text.value.clone()
                        } else {
                            unreachable!();
                        }
                    } else {
                        react
                            .create_interaction_response(&ctx, |response| {
                                response
                                    .kind(
                                        serenity::InteractionResponseType::ChannelMessageWithSource,
                                    )
                                    .interaction_response_data(|msg| msg.content("No cheating!"))
                            })
                            .await?;
                        return Ok(());
                    }
                }
            };

            ctx.channel_id()
                .send_message(ctx, |m| m.content(&something))
                .await?;
            Ok(())
        }
        Prefix(prefix_context) => {
            let something = something.unwrap_or("something".to_string());
            prefix_context.msg.delete(ctx).await?;
            ctx.channel_id()
                .send_message(ctx, |m| m.content(&something))
                .await?;
            Ok(())
        }
    }
}

/// Make the bot say 早安
#[poise::command(slash_command, prefix_command)]
async fn 早安(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("早安").await?;
    Ok(())
}

/// Make the bot ping to somewhere several times
#[poise::command(slash_command)]
async fn ping(
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

    let response = if authenticate(
        &ctx.serenity_context(),
        ResponsibleInteraction::ApplicationCommand(interaction),
        &format!("{}_{}", count, address.to_string()),
    )
    .await?
    {
        String::from_utf8_lossy(
            &Command::new("sh")
                .arg("-c")
                .arg(format!("ping -c{} {}", count, address))
                .output()
                .expect("Failed to perform ping command")
                .stdout,
        )
        .to_string()
    } else {
        "Nope, wrong passcode lol\n".to_string()
    };
    interaction
        .create_followup_message(&ctx, |msg| msg.ephemeral(true).content(response))
        .await?;
    Ok(())
}

/// Prints system information output by neofetch
#[poise::command(slash_command)]
async fn neofetch(
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

    let response = if authenticate(
        &ctx.serenity_context(),
        ResponsibleInteraction::ApplicationCommand(interaction),
        "neofetch",
    )
    .await?
    {
        String::from_utf8_lossy(
            &Command::new("sh")
                .arg("-c")
                .arg("neofetch --stdout")
                .output()
                .expect("Failed to get system information")
                .stdout,
        )
        .to_string()
    } else {
        "Nope, wrong passcode lol\n".to_string()
    };
    interaction
        .create_followup_message(&ctx, |msg| msg.ephemeral(true).content(response))
        .await?;
    Ok(())
}

/// Get a random fumo related message from the bot
#[poise::command(slash_command, prefix_command)]
async fn fumo(ctx: Context<'_>) -> Result<(), Error> {
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

/// Test command that require a string input
#[poise::command(slash_command)]
async fn test_input(ctx: Context<'_>) -> Result<(), Error> {
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
async fn test_auth(ctx: Context<'_>) -> Result<(), Error> {
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

/// Show a help menu
#[poise::command(slash_command, prefix_command)]
async fn help(
    ctx: Context<'_>,
    #[description = "Specific command to show help about"] command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration::default(),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                botinfo(),
                age(),
                say(),
                早安(),
                ping(),
                neofetch(),
                fumo(),
                test_input(),
                test_auth(),
                help(),
            ],
            event_handler: |ctx, event, _framework, data| Box::pin(handler(ctx, event, data)),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
