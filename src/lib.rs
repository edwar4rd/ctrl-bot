use poise::serenity_prelude::{self as serenity};

#[non_exhaustive]
pub enum ResponsibleInteraction<'a> {
    ApplicationCommand(&'a serenity::ApplicationCommandInteraction),
    MessageComponent(&'a serenity::MessageComponentInteraction),
    ModalSubmit(&'a serenity::ModalSubmitInteraction),
}

impl<'a> ResponsibleInteraction<'a> {
    pub async fn get_interaction_response(
        &self,
        http: impl AsRef<serenity::Http>,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.get_interaction_response(http).await
            }
            Self::MessageComponent(message_component) => {
                message_component.get_interaction_response(http).await
            }
            Self::ModalSubmit(modal_submit) => modal_submit.get_interaction_response(http).await,
        }
    }

    pub async fn create_interaction_response<F>(
        &self,
        http: impl AsRef<serenity::Http>,
        f: F,
    ) -> serenity::Result<()>
    where
        for<'b> F: FnOnce(
            &'b mut serenity::CreateInteractionResponse<'a>,
        ) -> &'b mut serenity::CreateInteractionResponse<'a>,
    {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .create_interaction_response(http, f)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component.create_interaction_response(http, f).await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit.create_interaction_response(http, f).await
            }
        }
    }

    pub async fn edit_original_interaction_response<F>(
        &self,
        http: impl AsRef<serenity::Http>,
        f: F,
    ) -> serenity::Result<serenity::Message>
    where
        F: FnOnce(&mut serenity::EditInteractionResponse) -> &mut serenity::EditInteractionResponse,
    {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .edit_original_interaction_response(http, f)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .edit_original_interaction_response(http, f)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit
                    .edit_original_interaction_response(http, f)
                    .await
            }
        }
    }

    pub async fn delete_original_interaction_response(
        &self,
        http: impl AsRef<serenity::Http>,
    ) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .delete_original_interaction_response(http)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .delete_original_interaction_response(http)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit
                    .delete_original_interaction_response(http)
                    .await
            }
        }
    }

    pub async fn create_followup_message<F>(
        &self,
        http: impl AsRef<serenity::Http>,
        f: F,
    ) -> serenity::Result<serenity::Message>
    where
        for<'b> F: FnOnce(
            &'b mut serenity::CreateInteractionResponseFollowup<'a>,
        ) -> &'b mut serenity::CreateInteractionResponseFollowup<'a>,
    {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command.create_followup_message(http, f).await
            }
            Self::MessageComponent(message_component) => {
                message_component.create_followup_message(http, f).await
            }
            Self::ModalSubmit(modal_submit) => modal_submit.create_followup_message(http, f).await,
        }
    }

    pub async fn edit_followup_message<F, M: Into<serenity::MessageId>>(
        &self,
        http: impl AsRef<serenity::Http>,
        message_id: M,
        f: F,
    ) -> serenity::Result<serenity::Message>
    where
        for<'b> F: FnOnce(
            &'b mut serenity::CreateInteractionResponseFollowup<'a>,
        ) -> &'b mut serenity::CreateInteractionResponseFollowup<'a>,
    {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .edit_followup_message(http, message_id, f)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .edit_followup_message(http, message_id, f)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit
                    .edit_followup_message(http, message_id, f)
                    .await
            }
        }
    }

    pub async fn delete_followup_message<M: Into<serenity::MessageId>>(
        &self,
        http: impl AsRef<serenity::Http>,
        message_id: M,
    ) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .delete_followup_message(http, message_id)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .delete_followup_message(http, message_id)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit.delete_followup_message(http, message_id).await
            }
        }
    }

    pub async fn get_followup_message<M: Into<serenity::MessageId>>(
        &self,
        http: impl AsRef<serenity::Http>,
        message_id: M,
    ) -> serenity::Result<serenity::Message> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .get_followup_message(http, message_id)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .get_followup_message(http, message_id)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                http.as_ref()
                    .get_followup_message(&modal_submit.token, message_id.into().into())
                    .await
            }
        }
    }

    pub async fn defer(&self, http: impl AsRef<serenity::Http>) -> serenity::Result<()> {
        match &self {
            Self::ApplicationCommand(application_command) => {
                application_command
                    .defer(http)
                    .await
            }
            Self::MessageComponent(message_component) => {
                message_component
                    .defer(http)
                    .await
            }
            Self::ModalSubmit(modal_submit) => {
                modal_submit
                    .defer(http)
                    .await
            }
        }
    }
}
