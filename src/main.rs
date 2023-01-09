use dc_bot::prelude::*;

#[cfg(feature = "handler")]
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
                        #[cfg(feature = "modal_tests")]
                        if interaction.data.custom_id == "test_auth.auth_btn" {
                            commands::tests::test_auth_btn_handler(
                                ctx,
                                &ResponsibleInteraction::MessageComponent(&interaction),
                            )
                            .await?
                        }
                        #[cfg(feature = "tools")]
                        if interaction.data.custom_id == "neofetch.btn" {
                            commands::tools::neofetch_btn_handler(
                                ctx,
                                &ResponsibleInteraction::MessageComponent(&interaction),
                            )
                            .await?
                        }
                        #[cfg(feature = "tools")]
                        if interaction.data.custom_id == "stop.btn" {
                            commands::tools::stop_btn_handler(
                                ctx,
                                &ResponsibleInteraction::MessageComponent(&interaction),
                            )
                            .await?
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

#[tokio::main]
async fn main() {
    let mut commands = vec![];

    #[cfg(feature = "random")]
    {
        commands.push(commands::random::fumo());
        commands.push(commands::random::say());
        commands.push(commands::random::早安());
    }

    #[cfg(feature = "stdio_tests")]
    {
        commands.push(commands::tests::ask());
        commands.push(commands::tests::msg());
        commands.push(commands::tests::getline());
    }

    #[cfg(feature = "modal_tests")]
    {
        commands.push(commands::tests::test_input());
        commands.push(commands::tests::test_auth());
    }

    #[cfg(feature = "tools")]
    {
        commands.push(commands::tools::neofetch());
        commands.push(commands::tools::ping());
        commands.push(commands::tools::stop());
    }

    #[cfg(feature = "dcbothub")]
    {
        commands.push(commands::dcbothub::bothub());
    }

    commands.push(commands::help());
    commands.push(commands::botinfo());

    #[allow(unused_mut)]
    let mut options = poise::FrameworkOptions {
        commands,
        ..Default::default()
    };

    #[cfg(feature = "handler")]
    {
        options.event_handler = |ctx, event, _framework, data| Box::pin(handler(ctx, event, data));
    }

    let framework = poise::Framework::builder()
        .options(options)
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                ctx.set_presence(
                    Some(serenity::Activity::watching("學測倒數多少天 👀")),
                    serenity::OnlineStatus::Idle,
                )
                .await;
                ctx.data
                    .write()
                    .await
                    .insert::<ShardManagerContainer>(std::sync::Arc::clone(
                        &framework.shard_manager(),
                    ));
                Ok(Data {
                    stdio_lock: tokio::sync::Mutex::new(()),
                    stdin_linereader: tokio::sync::Mutex::new(tokio_util::codec::FramedRead::new(
                        tokio::io::stdin(),
                        tokio_util::codec::LinesCodec::new(),
                    )),
                })
            })
        });

    framework.run().await.unwrap();
    eprintln!(
        "Bot stopped at {}...",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );
}
