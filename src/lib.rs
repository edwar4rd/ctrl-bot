pub mod commands;

mod interactions;
pub use interactions::ResponsibleInteraction;

pub mod auth;

pub mod prelude {
    pub use crate::auth;
    pub use crate::commands;
    pub use poise::{
        serenity_prelude as serenity,
        Context::{Application, Prefix},
    };
    pub use rand::prelude::*;
    pub use std::net::IpAddr;
    // pub use sha3::{Digest, Sha3_512};
    pub use crate::ResponsibleInteraction;
    pub use poise::ApplicationCommandOrAutocompleteInteraction;
    pub use std::process::Command;
    pub use std::time::Duration;

    pub type Error = Box<dyn std::error::Error + Send + Sync>;
    pub type Context<'a> = poise::Context<'a, Data, Error>;
    pub struct Data {
        pub stdio_lock: tokio::sync::Mutex<()>,
    }
    pub struct ShardManagerContainer;

    impl serenity::TypeMapKey for ShardManagerContainer {
        type Value = std::sync::Arc<tokio::sync::Mutex<serenity::ShardManager>>;
    }
}
