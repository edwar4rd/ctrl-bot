use poise::futures_util::StreamExt;
use tokio::time::timeout;

use crate::prelude::*;

/// Execute commands of parent dcbothub process
#[poise::command(
    slash_command,
    prefix_command,
    subcommands(
        "list",
        "list_status",
        "list_tasks",
    )
)]
pub async fn bothub(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// list name of all bots loaded from bots.toml each in a line
#[poise::command(slash_command, prefix_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let stdio_lock = match timeout(
        tokio::time::Duration::from_secs(30),
        ctx.data().stdio_lock.lock(),
    )
    .await
    {
        Ok(lock) => lock,
        Err(_) => {
            ctx.say("Failed getting data from bothub.").await?;
            return Ok(());
        }
    };

    let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
    println!("list");
    stdin_reader
        .next()
        .await
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let bot_list = stdin_reader.next().await.unwrap().unwrap();
    ctx.say(format!(
        "Loaded bots: \n- `{}\n`",
        bot_list.replace(' ', "`\n- `")
    ))
    .await?;

    drop(stdin_reader);
    drop(stdio_lock);
    Ok(())
}

/// list every running/exited bot in a line with name and status listed
#[poise::command(slash_command, prefix_command, rename = "list-status")]
async fn list_status(ctx: Context<'_>) -> Result<(), Error> {
    let stdio_lock = match timeout(
        tokio::time::Duration::from_secs(30),
        ctx.data().stdio_lock.lock(),
    )
    .await
    {
        Ok(lock) => lock,
        Err(_) => {
            ctx.say("Failed getting data from bothub.").await?;
            return Ok(());
        }
    };
    let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
    println!("list-status");
    let line_count = stdin_reader
        .next()
        .await
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut bot_list = String::new();
    for _ in 0..line_count {
        bot_list.push_str(&stdin_reader.next().await.unwrap().unwrap());
        bot_list.push('\n');
    }

    ctx.say(format!("Bot instance status: ```\n{}```", bot_list))
        .await?;

    drop(stdin_reader);
    drop(stdio_lock);
    Ok(())
}

/// list running/finished tasks such as build processes or pull processes
#[poise::command(slash_command, prefix_command, rename = "list-tasks")]
async fn list_tasks(ctx: Context<'_>) -> Result<(), Error> {
    let stdio_lock = match timeout(
        tokio::time::Duration::from_secs(30),
        ctx.data().stdio_lock.lock(),
    )
    .await
    {
        Ok(lock) => lock,
        Err(_) => {
            ctx.say("Failed getting data from bothub.").await?;
            return Ok(());
        }
    };
    let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
    println!("list-tasks");
    let line_count = stdin_reader
        .next()
        .await
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut bot_list = String::new();
    for _ in 0..line_count {
        bot_list.push_str(&stdin_reader.next().await.unwrap().unwrap());
        bot_list.push('\n');
    }

    ctx.say(format!("Task status: ```\n{}```", bot_list))
        .await?;

    drop(stdin_reader);
    drop(stdio_lock);
    Ok(())
}
