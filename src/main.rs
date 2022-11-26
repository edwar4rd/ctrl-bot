use poise::{
    serenity_prelude as serenity,
    Context::{Application, Prefix},
};
use rand::prelude::*;
use std::time::Duration;
// use std::process::Command;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
// User data, which is stored and accessible in all command invocations
struct Data {}

#[poise::command(slash_command, prefix_command)]
async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("version = 0.0.5\nlast-update ~= 20221126 14:00")
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
    something: String,
) -> Result<(), Error> {
    match ctx {
        Application(_) => {
            ctx.defer_ephemeral().await?;
            ctx.say("Your message is being sent...").await?;
            ctx.channel_id()
                .send_message(ctx, |m| m.content(&something))
                .await?;
            Ok(())
        }
        Prefix(prefix_context) => {
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

/*
/// Make the bot ping to somewhere four times
#[poise::command(slash_command, prefix_command)]
async fn ping(
    ctx: Context<'_>,
    #[description = "echo neofetch $(date +%s | cut -c-8) $DISCORD_UID $PASSWORD | sha512sum | cut -c-128"]
    passcode: String,
    #[description = "something.something.something.something"] address: String,
) -> Result<(), Error> {
    let response = if passcode.trim().starts_with(
        String::from_utf8_lossy(
            &Command::new("sh")
                .arg("-c")
                // .env("PASSWORD", "phrase drift yiss ektjed displays jour yiyq")
                .env("DISCORD_UID", ctx.author().id.to_string())
                .env("BOT_COMMAND", "ping")
                .arg("echo $BOT_COMMAND $(date +%s | cut -c-8) $DISCORD_UID $PASSWORD | sha512sum | cut -c-128")
                .output()
                .expect("Failed to hash user passcode")
                .stdout,
        )
        .trim(),
    ) {
        let address = address.trim().split(".").collect::<Vec<&str>>();
        if address.len() != 4 {
            ctx.say("Nope, don't use IPv6 or mess with me").await?;
            return Ok(())
        }
        let address = (
            match address[0].parse::<u8>() {
                Ok(a0) => a0,
                Err(_) => {
                    ctx.say("Nope, don't use IPv6 or mess with me").await?;
                    return Ok(())
                }
            },
            match address[1].parse::<u8>() {
                Ok(a1) => a1,
                Err(_) => {
                    ctx.say("Nope, don't use IPv6 or mess with me").await?;
                    return Ok(())
                }
            },
            match address[2].parse::<u8>() {
                Ok(a2) => a2,
                Err(_) => {
                    ctx.say("Nope, don't use IPv6 or mess with me").await?;
                    return Ok(())
                }
            },
            match address[3].parse::<u8>() {
                Ok(a3) => a3,
                Err(_) => {
                    ctx.say("Nope, don't use IPv6 or mess with me").await?;
                    return Ok(())
                }
            }
        );
        String::from_utf8_lossy(
            &Command::new("sh")
                .arg("-c")
                .arg(format!("ping -c1 {}.{}.{}.{}", address.0, address.1, address.2, address.3))
                .output()
                .expect("Failed to get system information")
                .stdout,
        ).to_string()
    } else {
        "Nope, wrong passcode lol\n".to_string()
    };
    ctx.say(response).await?;
    Ok(())
}
*/

/*
/// Prints system information output by neofetch
#[poise::command(slash_command, prefix_command)]
async fn neofetch(
    ctx: Context<'_>,
    #[description = "echo neofetch $(date +%s | cut -c-8) $DISCORD_UID $PASSWORD | sha512sum | cut -c-128"]
    passcode: String,
    // #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let response = if passcode.trim().starts_with(
        String::from_utf8_lossy(
            &Command::new("sh")
                .arg("-c")
                .env("PASSWORD", "phrase drift yiss ektjed displays jour yiyq")
                .env("DISCORD_UID", ctx.author().id.to_string())
                .env("BOT_COMMAND", "neofetch")
                .arg("echo $BOT_COMMAND $(date +%s | cut -c-8) $DISCORD_UID $PASSWORD | sha512sum | cut -c-128")
                .output()
                .expect("Failed to hash user passcode")
                .stdout,
        )
        .trim(),
    ) {
        String::from_utf8_lossy(
            &Command::new("sh")
                .arg("-c")
                .arg("neofetch --stdout")
                .output()
                .expect("Failed to get system information")
                .stdout,
        ).to_string()
    } else {
        "Nope, wrong passcode lol\n".to_string()
    };
    ctx.say(response).await?;
    Ok(())
}
*/

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
            msg.edit(&ctx, |m| m.components(|c| c)).await.unwrap();
            return Ok(());
        }
    };
    msg.edit(&ctx, |m| m.components(|c| c)).await.unwrap();
    if react.user.id == ctx.author().id {
        react
            .create_interaction_response(&ctx, |r| {
                r.kind(serenity::InteractionResponseType::Modal);
                r.interaction_response_data(|d| {
                    d.custom_id("test_input.modal");
                    d.title("1+1 = ?");
                    d.components(|c| {
                        c.create_action_row(|ar| {
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
            .await
            .unwrap();

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
                /*ping(), neofetch(), */ fumo(),
                test_input(),
                help(),
            ],
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
