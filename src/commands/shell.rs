use crate::prelude::*;

/// Execute a arbitrary command and return it's output
#[poise::command(slash_command)]
pub async fn shell(
    ctx: Context<'_>,
    #[description = "command to be executed "] command: String,
) -> Result<(), Error> {
    let interaction = slash_ctx_as_responsibe_interaction(&ctx);
    if !auth::authenticate(
        ctx.serenity_context(),
        &interaction,
        &format!("shell_{command}"),
    )
    .await?
    {
        interaction
            .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
            .await?;
        return Ok(());
    }
    interaction
        .create_followup_message(&ctx, |msg| {
            msg.ephemeral(true).content("Executing command...")
        })
        .await?;
    let child = std::process::Command::new("bash")
        .arg("-c")
        .arg(command)
        .output();
    interaction
        .create_followup_message(&ctx, |msg| {
            msg.ephemeral(true).content(match child {
                Ok(output) => format!(
                    "Stdout: \n```{}```\nStderr: \n```{}```\n",
                    if std::str::from_utf8(&output.stdout).unwrap().trim().len() > 0 {
                        std::str::from_utf8(&output.stdout).unwrap()
                    } else {
                        "(None)\n"
                    },
                    if std::str::from_utf8(&output.stderr).unwrap().trim().len() > 0 {
                        std::str::from_utf8(&output.stderr).unwrap()
                    } else {
                        "(None)\n"
                    },
                ),
                Err(_) => "Command failed to execute!".to_string(),
            })
        })
        .await?;
    Ok(())
}
