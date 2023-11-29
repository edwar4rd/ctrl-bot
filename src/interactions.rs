use poise::serenity_prelude::{
    CreateInteractionResponse, CreateInteractionResponseFollowup, EditInteractionResponse,
};

use crate::prelude::*;

pub enum ResponsibleInteraction<'a> {
    ApplicationCommand(&'a serenity::CommandInteraction),
    MessageComponent(&'a serenity::ComponentInteraction),
    ModalSubmit(&'a serenity::ModalInteraction),
}

impl<'a> ResponsibleInteraction<'a> {
    pub async fn get_response(
        &self,
        http: impl AsRef<serenity::Http>,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.get_response(http).await
            }
            Self::MessageComponent(message_component) => message_component.get_response(http).await,
            Self::ModalSubmit(modal_submit) => modal_submit.get_response(http).await,
        }
    }

    pub async fn create_response(
        &self,
        cache_http: impl serenity::CacheHttp,
        builder: CreateInteractionResponse,
    ) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .create_response(cache_http, builder)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component.create_response(cache_http, builder).await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit.create_response(cache_http, builder).await
            }
        }
    }

    pub async fn edit_response(
        &self,
        cache_http: impl serenity::CacheHttp,
        builder: EditInteractionResponse,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.edit_response(cache_http, builder).await
            }
            Self::MessageComponent(message_component) => {
                message_component.edit_response(cache_http, builder).await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit.edit_response(cache_http, builder).await
            }
        }
    }

    pub async fn delete_response(&self, http: impl AsRef<serenity::Http>) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.delete_response(http).await
            }
            Self::MessageComponent(message_component) => {
                message_component.delete_response(http).await
            }
            Self::ModalSubmit(modal_submit) => modal_submit.delete_response(http).await,
        }
    }

    pub async fn create_followup(
        &self,
        cache_http: impl serenity::CacheHttp,
        builder: CreateInteractionResponseFollowup,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .create_followup(cache_http, builder)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component.create_followup(cache_http, builder).await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit.create_followup(cache_http, builder).await
            }
        }
    }

    pub async fn edit_followup<M: Into<serenity::MessageId>>(
        &self,
        cache_http: impl serenity::CacheHttp,
        message_id: M,
        builder: CreateInteractionResponseFollowup,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .edit_followup(cache_http, message_id, builder)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .edit_followup(cache_http, message_id, builder)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit
                    .edit_followup(cache_http, message_id, builder)
                    .await
            }
        }
    }

    pub async fn delete_followup<M: Into<serenity::MessageId>>(
        &self,
        http: impl AsRef<serenity::Http>,
        message_id: M,
    ) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.delete_followup(http, message_id).await
            }
            Self::MessageComponent(message_component) => {
                message_component.delete_followup(http, message_id).await
            }
            Self::ModalSubmit(modal_submit) => modal_submit.delete_followup(http, message_id).await,
        }
    }

    pub async fn get_followup<M: Into<serenity::MessageId>>(
        &self,
        http: impl AsRef<serenity::Http>,
        message_id: M,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.get_followup(http, message_id).await
            }
            Self::MessageComponent(message_component) => {
                message_component.get_followup(http, message_id).await
            }
            Self::ModalSubmit(modal_submit) => {
                http.as_ref()
                    .get_followup_message(&modal_submit.token, message_id.into().into())
                    .await
            }
        }
    }

    pub async fn defer(&self, cache_http: impl serenity::CacheHttp) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.defer(cache_http).await
            }
            Self::MessageComponent(message_component) => message_component.defer(cache_http).await,
            Self::ModalSubmit(modal_submit) => modal_submit.defer(cache_http).await,
        }
    }

    pub fn user(&self) -> &'a serenity::User {
        match &self {
            Self::ApplicationCommand(application_command) => &application_command.user,
            Self::MessageComponent(message_component) => &message_component.user,
            Self::ModalSubmit(modal_submit) => &modal_submit.user,
        }
    }
}
