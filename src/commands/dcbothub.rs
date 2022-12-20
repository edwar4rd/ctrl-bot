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
        "status",
        // "task-status",
        // "clean",
        // "build",
        // "pull",
        // "start",
        // "msg",
        // "kill",
        // "exit",
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

/// check the status of a bot
#[poise::command(slash_command)]
async fn status(
    ctx: Context<'_>,
    #[description = "Name of the bot of interest"]
    #[autocomplete = "autocomplete_botname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_botname(ctx, "").await;
    if candidates.iter().any(|name| *name == botname) {
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
        let bot_status = {
            println!("status {}", botname);
            stdin_reader
                .next()
                .await
                .unwrap()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            stdin_reader.next().await.unwrap().unwrap()
        };
        drop(stdin_reader);
        drop(stdio_lock);

        ctx.say(format!("Bot status:\n```\n{}```", bot_status))
            .await?;
    } else {
        ctx.say(format!("Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.", botname))
            .await?;
    }

    Ok(())
}

async fn autocomplete_botname<'a>(ctx: Context<'_>, partial: &'a str) -> Vec<String> {
    let bot_list = if let Ok(stdio_lock) = timeout(
        tokio::time::Duration::from_secs(30),
        ctx.data().stdio_lock.lock(),
    )
    .await
    {
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
        drop(stdin_reader);
        drop(stdio_lock);
        bot_list
    } else {
        "".to_string()
    };

    // futures::stream::iter().to_owned().map(|name| name.to_string())
    bot_list
        .split(' ')
        .filter(|name| name.starts_with(partial))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}
