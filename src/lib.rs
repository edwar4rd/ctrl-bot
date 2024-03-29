pub mod commands;

mod interactions;
pub use interactions::ResponsibleInteraction;

pub mod auth;

pub mod prelude {
    pub use crate::auth;
    pub use crate::commands;
    use poise::CommandInteractionType;
    pub use poise::{
        serenity_prelude as serenity,
        Context::{Application, Prefix},
    };
    pub use rand::prelude::*;
    pub use std::net::IpAddr;
    // pub use sha3::{Digest, Sha3_512};
    pub use crate::ResponsibleInteraction;
    pub use std::process::Command;
    pub use std::time::Duration;

    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Context<'a> = poise::Context<'a, Data, Error>;
    pub struct Data {
        pub stdio_lock: tokio::sync::Mutex<()>,
        pub stdin_linereader: tokio::sync::Mutex<
            tokio_util::codec::FramedRead<tokio::io::Stdin, tokio_util::codec::LinesCodec>,
        >,
    }
    pub struct ShardManagerContainer;

    impl serenity::prelude::TypeMapKey for ShardManagerContainer {
        type Value = std::sync::Arc<serenity::ShardManager>;
    }

    pub fn slash_ctx_as_responsibe_interaction<'a>(
        ctx: &'a Context<'_>,
    ) -> ResponsibleInteraction<'a> {
        match &ctx {
            Application(application_context) => {
                assert_eq!(
                    application_context.interaction_type,
                    CommandInteractionType::Command
                );
                ResponsibleInteraction::ApplicationCommand(application_context.interaction)
            }
            _ => {
                unreachable!();
            }
        }
    }

    pub async fn autosplit_output<'a>(
        ctx: &'a Context<'_>,
        interaction: &'a ResponsibleInteraction<'a>,
        content: &'a String,
    ) -> Result<(), Error> {
        let response = if content.len() > 1990 {
            let mut lines = content.lines();
            let mut current = String::new();
            loop {
                match lines.next() {
                    Some(line) => {
                        if current.len() + line.len() > 1990 {
                            interaction
                                .create_followup(
                                    &ctx,
                                    serenity::CreateInteractionResponseFollowup::new()
                                        .ephemeral(true)
                                        .content(format!("```{}```", current)),
                                )
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
            format!("```{content}```")
        };
        interaction
            .create_followup(
                &ctx,
                serenity::CreateInteractionResponseFollowup::new()
                    .ephemeral(true)
                    .content(response),
            )
            .await?;
        Ok(())
    }

    pub fn default_auth_fail_response() -> serenity::CreateInteractionResponseFollowup {
        serenity::CreateInteractionResponseFollowup::new()
            .ephemeral(true)
            .content("Nope!\n")
    }
}
