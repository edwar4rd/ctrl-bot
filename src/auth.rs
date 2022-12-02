// Helper function that verify the authenticity of a action

use crate::prelude::*;
use rsa::{pkcs8::DecodePublicKey, PaddingScheme, PublicKey};
use std::iter;
use std::time;

pub async fn authenticate<'a>(
    ctx: &serenity::Context,
    interaction: &ResponsibleInteraction<'a>,
    action_data: &str,
) -> Result<bool, Error> {
    let challenge_random = iter::repeat_with(|| thread_rng().gen::<u8>())
        .take(192 * 2)
        .collect::<Vec<u8>>();
    let challenge_encoded = format!(
        "{}_{}_{}_{}\n",
        base64::encode(&challenge_random),
        interaction.user(),
        time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
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
