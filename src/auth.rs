// Helper function that verify the authenticity of a action

use crate::prelude::*;
use base64::Engine;
use rsa::pkcs1v15::Pkcs1v15Sign;
use rsa::{pkcs8::DecodePublicKey, RsaPublicKey};
use sha3::{Digest, Sha3_512};
use std::iter;
use std::time;

pub async fn authenticate<'a>(
    ctx: &serenity::Context,
    interaction: &ResponsibleInteraction<'a>,
    action_data: &str,
) -> Result<bool, Error> {
    let challenge_random = iter::repeat_with(|| thread_rng().gen::<u8>())
        .take(192)
        .collect::<Vec<u8>>();
    let challenge_encoded = format!(
        "{}_{}_{}_{}\n",
        base64::engine::general_purpose::STANDARD.encode(&challenge_random),
        interaction.user(),
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
        action_data
    );

    let challenge_message = format!("Please authenticate by signing this random message(including the newline) with your secret key:
    ```{challenge_encoded}```
    
    ```openssl dgst -sha3-512 -sign privkey.pem | openssl enc -base64```");
    let challenge_components: Vec<serenity::CreateActionRow> =
        vec![serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new("authenticate.button")
                .label("Submit signed")
                .style(serenity::ButtonStyle::Primary),
        ])];
    let challenge_message = serenity::CreateInteractionResponseMessage::new()
        .ephemeral(true)
        .content(challenge_message)
        .components(challenge_components);
    let challenge_builder = serenity::CreateInteractionResponse::Message(challenge_message);
    interaction.create_response(&ctx, challenge_builder).await?;

    let too_slow_response_followup: serenity::CreateInteractionResponseFollowup =
        serenity::CreateInteractionResponseFollowup::new()
            .ephemeral(true)
            .content("(Too slow!!)");
    let succeeded_response: serenity::CreateInteractionResponse =
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("(Authentication Succeeded!)"),
        );
    let failed_response: serenity::CreateInteractionResponse =
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("(Authentication Failed!)"),
        );
    let fail_to_parse_response: serenity::CreateInteractionResponse =
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("(Failed to parse responsed message)"),
        );

    let btn_reply = interaction.get_response(&ctx).await?;

    let btn_reply_react = match btn_reply
        .await_component_interaction(&ctx)
        .timeout(Duration::from_secs(60))
        .await
    {
        Some(react) => react,
        None => {
            interaction
                .create_followup(&ctx, too_slow_response_followup)
                .await?;
            return Ok(false);
        }
    };

    let auth_modal_components: Vec<serenity::CreateActionRow> =
        vec![serenity::CreateActionRow::InputText(
            serenity::CreateInputText::new(
                serenity::InputTextStyle::Paragraph,
                "signed message",
                "authenticate.modal.signed",
            )
            .required(true),
        )];
    let auth_modal: serenity::CreateModal = serenity::CreateModal::new(
        "authenticate.modal",
        "Submit signed message to authenticate",
    )
    .components(auth_modal_components);
    let auth_response: serenity::CreateInteractionResponse =
        serenity::CreateInteractionResponse::Modal(auth_modal);

    btn_reply_react.create_response(&ctx, auth_response).await?;

    let modal_reply_react = match btn_reply
        .await_modal_interaction(&ctx)
        .timeout(Duration::from_secs(60))
        .await
    {
        Some(react) => react,
        None => {
            btn_reply_react
                .create_followup(&ctx, too_slow_response_followup)
                .await?;
            return Ok(false);
        }
    };

    if let serenity::ActionRowComponent::InputText(text) =
        &modal_reply_react.data.components[0].components[0]
    {
        if let Ok(signed) = base64::engine::general_purpose::STANDARD.decode(
            &text
                .value
                .as_ref()
                .unwrap_or(&String::default())
                .trim()
                .as_bytes()
                .into_iter()
                .filter(|c| !c.is_ascii_whitespace())
                .map(|x| *x)
                .collect::<Vec<u8>>(),
        ) {
            let mut my_hasher = Sha3_512::new();
            my_hasher.update(challenge_encoded);
            let hashed = my_hasher.finalize();
            if RsaPublicKey::from_public_key_pem(
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
            .verify(Pkcs1v15Sign::new::<Sha3_512>(), &hashed, &signed)
            .is_ok()
            {
                modal_reply_react
                    .create_response(&ctx, succeeded_response)
                    .await?;
                Ok(true)
            } else {
                modal_reply_react
                    .create_response(&ctx, failed_response)
                    .await?;
                Ok(false)
            }
        } else {
            modal_reply_react
                .create_response(&ctx, fail_to_parse_response)
                .await?;
            Ok(false)
        }
    } else {
        unreachable!();
    }
}
