use crate::prelude::*;

/// Show a help menu
#[poise::command(slash_command, prefix_command)]
pub async fn help(
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

/// Displays information about the bot
#[poise::command(slash_command, prefix_command)]
pub async fn botinfo(ctx: Context<'_>) -> Result<(), Error> {
    use build_time::build_time_local;

    const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
    let botinfo_message: poise::CreateReply = poise::CreateReply::default()
        .ephemeral(true)
        .content(format!(
            "```version = {}\nbuild-time = {}```",
            VERSION.unwrap_or("UNKNOWN"),
            build_time_local!("%Y-%m-%d %H:%M:%S %:z")
        ));
    ctx.send(botinfo_message).await?;
    Ok(())
}

#[cfg(feature = "random")]
pub mod random;

#[cfg(any(feature = "stdio_tests", feature = "modal_tests"))]
pub mod tests;

#[cfg(feature = "tools")]
pub mod tools;

#[cfg(feature = "dcbothub")]
pub mod dcbothub;

#[cfg(feature = "shell")]
pub mod shell;

// #[cfg(all(feature = "stdio_tests", feature = "dcbothub"))]
// compile_error!("feature \"stdio_tests\" and feature \"dcbothub\" shouldn't be enabled at the same time");
