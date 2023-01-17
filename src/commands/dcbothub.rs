use poise::futures_util::StreamExt;
use tokio::time::timeout;

use crate::prelude::*;

/// Execute commands of parent dcbothub process
#[poise::command(
    slash_command,
    prefix_command,
    subcommands(
        "list",
        "list_existing",
        "list_executing",
        "list_status",
        "list_tasks",
        "status",
        "task_status",
        "clean",
        "build",
        "pull",
        "start",
        // "msg",
        // "verify",
        "kill",
        "control_restart",
        "terminate",
        // "conclude",
        // "wait",
        // "finish",
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
    if bot_list != "" {
        ctx.say(format!(
            "Loaded bots: \n- `{}\n`",
            bot_list.replace(' ', "`\n- `")
        ))
        .await?;
    } else {
        ctx.say("None").await?;
    }

    drop(stdin_reader);
    drop(stdio_lock);
    Ok(())
}

/// list every running/exited bot in a line
#[poise::command(slash_command, prefix_command, rename = "list-existing")]
async fn list_existing(ctx: Context<'_>) -> Result<(), Error> {
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
    println!("list-existing");
    stdin_reader
        .next()
        .await
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let bot_list = stdin_reader.next().await.unwrap().unwrap();
    if bot_list != "" {
        ctx.say(format!(
            "Existing bots: \n- `{}\n`",
            bot_list.replace(' ', "`\n- `")
        ))
        .await?;
    } else {
        ctx.say("None").await?;
    }

    drop(stdin_reader);
    drop(stdio_lock);
    Ok(())
}

/// list every running/exited task in a line
#[poise::command(slash_command, prefix_command, rename = "list-executing")]
async fn list_executing(ctx: Context<'_>) -> Result<(), Error> {
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
    println!("list-executing");
    stdin_reader
        .next()
        .await
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let task_list = stdin_reader.next().await.unwrap().unwrap();
    if task_list != "" {
        ctx.say(format!(
            "Tasks: \n- `{}\n`",
            task_list.replace(' ', "`\n- `")
        ))
        .await?;
    } else {
        ctx.say("None").await?;
    }

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
    #[autocomplete = "autocomplete_ebotname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_ebotname(ctx, "").await;
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
        ctx.say(format!(
            "Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.",
            botname
        ))
        .await?;
    }

    Ok(())
}

/// check the status of a task
#[poise::command(slash_command, rename = "task-status")]
async fn task_status(
    ctx: Context<'_>,
    #[description = "Id of the task of interest"]
    #[autocomplete = "autocomplete_taskid"]
    taskid: String,
) -> Result<(), Error> {
    let candidates = autocomplete_taskid(ctx, "").await;
    if candidates.iter().any(|name| *name == taskid) {
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
            println!("task-status {}", taskid);
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

        ctx.say(format!("Task status:\n```\n{}```", bot_status))
            .await?;
    } else {
        ctx.say(format!(
            "Task\n```\n{}\n```isn't found in task list and is filtered for security reasons.",
            taskid
        ))
        .await?;
    }

    Ok(())
}

/// perform a `cargo clean` at the repo of a bot
#[poise::command(slash_command)]
async fn clean(
    ctx: Context<'_>,
    #[description = "Name of the bot of interest"]
    #[autocomplete = "autocomplete_botname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_botname(ctx, "").await;
    if candidates.iter().any(|name| *name == botname) {
        let interaction = slash_ctx_as_responsibe_interaction(&ctx);
        if !auth::authenticate(
            ctx.serenity_context(),
            &interaction,
            &format!("bothub_clean_{botname}"),
        )
        .await?
        {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
                .await?;
            return Ok(());
        }

        let stdio_lock = match timeout(
            tokio::time::Duration::from_secs(30),
            ctx.data().stdio_lock.lock(),
        )
        .await
        {
            Ok(lock) => lock,
            Err(_) => {
                interaction
                    .create_followup_message(ctx, |m| {
                        m.ephemeral(true).content("Failed sending command to bothub.")
                    })
                    .await?;
                return Ok(());
            }
        };

        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        let command_result = {
            println!("clean {}", botname);
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

        interaction
            .create_followup_message(ctx, |m| {
                m.ephemeral(true).content(format!("Result:\n```\n{}```", command_result))
            })
            .await?;
    } else {
        ctx.say(format!(
            "Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.",
            botname
        ))
        .await?;
    }

    Ok(())
}

/// perform a `cargo build` at the repo of a bot
#[poise::command(slash_command)]
async fn build(
    ctx: Context<'_>,
    #[description = "Name of the bot of interest"]
    #[autocomplete = "autocomplete_botname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_botname(ctx, "").await;
    if candidates.iter().any(|name| *name == botname) {
        let interaction = slash_ctx_as_responsibe_interaction(&ctx);
        if !auth::authenticate(
            ctx.serenity_context(),
            &interaction,
            &format!("bothub_build_{botname}"),
        )
        .await?
        {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
                .await?;
            return Ok(());
        }

        let stdio_lock = match timeout(
            tokio::time::Duration::from_secs(30),
            ctx.data().stdio_lock.lock(),
        )
        .await
        {
            Ok(lock) => lock,
            Err(_) => {
                interaction
                    .create_followup_message(ctx, |m| {
                        m.ephemeral(true).content("Failed sending command to bothub.")
                    })
                    .await?;
                return Ok(());
            }
        };

        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        let command_result = {
            println!("build {}", botname);
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

        interaction
            .create_followup_message(ctx, |m| {
                m.ephemeral(true).content(format!("Result:\n```\n{}```", command_result))
            })
            .await?;
    } else {
        ctx.say(format!(
            "Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.",
            botname
        ))
        .await?;
    }

    Ok(())
}

/// perform a `git pull` at the repo of a bot
#[poise::command(slash_command)]
async fn pull(
    ctx: Context<'_>,
    #[description = "Name of the bot of interest"]
    #[autocomplete = "autocomplete_botname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_botname(ctx, "").await;
    if candidates.iter().any(|name| *name == botname) {
        let interaction = slash_ctx_as_responsibe_interaction(&ctx);
        if !auth::authenticate(
            ctx.serenity_context(),
            &interaction,
            &format!("bothub_pull_{botname}"),
        )
        .await?
        {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
                .await?;
            return Ok(());
        }

        let stdio_lock = match timeout(
            tokio::time::Duration::from_secs(30),
            ctx.data().stdio_lock.lock(),
        )
        .await
        {
            Ok(lock) => lock,
            Err(_) => {
                interaction
                    .create_followup_message(ctx, |m| {
                        m.ephemeral(true).content("Failed sending command to bothub.")
                    })
                    .await?;
                return Ok(());
            }
        };

        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        let command_result = {
            println!("pull {}", botname);
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

        interaction
            .create_followup_message(ctx, |m| {
                m.ephemeral(true).content(format!("Result:\n```\n{}```", command_result))
            })
            .await?;
    } else {
        ctx.say(format!(
            "Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.",
            botname
        ))
        .await?;
    }

    Ok(())
}

/// start the bot if it isn't already runninng
#[poise::command(slash_command)]
async fn start(
    ctx: Context<'_>,
    #[description = "Name of the bot of interest"]
    #[autocomplete = "autocomplete_botname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_botname(ctx, "").await;
    if candidates.iter().any(|name| *name == botname) {
        let interaction = slash_ctx_as_responsibe_interaction(&ctx);
        if !auth::authenticate(
            ctx.serenity_context(),
            &interaction,
            &format!("bothub_start_{botname}"),
        )
        .await?
        {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
                .await?;
            return Ok(());
        }

        let stdio_lock = match timeout(
            tokio::time::Duration::from_secs(30),
            ctx.data().stdio_lock.lock(),
        )
        .await
        {
            Ok(lock) => lock,
            Err(_) => {
                interaction
                    .create_followup_message(ctx, |m| {
                        m.ephemeral(true).content("Failed sending command to bothub.")
                    })
                    .await?;
                return Ok(());
            }
        };

        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        let command_result = {
            println!("start {}", botname);
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

        interaction
            .create_followup_message(ctx, |m| {
                m.ephemeral(true).content(format!("Result:\n```\n{}```", command_result))
            })
            .await?;
    } else {
        ctx.say(format!(
            "Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.",
            botname
        ))
        .await?;
    }

    Ok(())
}

/// stop a bot with the given name
#[poise::command(slash_command)]
async fn kill(
    ctx: Context<'_>,
    #[description = "Name of the bot of interest"]
    #[autocomplete = "autocomplete_ebotname"]
    botname: String,
) -> Result<(), Error> {
    let candidates = autocomplete_ebotname(ctx, "").await;
    if candidates.iter().any(|name| *name == botname) {
        let interaction = slash_ctx_as_responsibe_interaction(&ctx);
        if !auth::authenticate(
            ctx.serenity_context(),
            &interaction,
            &format!("bothub_kill_{botname}"),
        )
        .await?
        {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
                .await?;
            return Ok(());
        }

        let stdio_lock = match timeout(
            tokio::time::Duration::from_secs(30),
            ctx.data().stdio_lock.lock(),
        )
        .await
        {
            Ok(lock) => lock,
            Err(_) => {
                interaction
                    .create_followup_message(ctx, |m| {
                        m.ephemeral(true).content("Failed sending command to bothub.")
                    })
                    .await?;
                return Ok(());
            }
        };

        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        let command_result = {
            println!("kill {}", botname);
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

        interaction
            .create_followup_message(ctx, |m| {
                m.ephemeral(true).content(format!("Result:\n```\n{}```", command_result))
            })
            .await?;
    } else {
        ctx.say(format!(
            "Bot\n```\n{}\n```isn't found in bot list and is filtered for security reasons.",
            botname
        ))
        .await?;
    }

    Ok(())
}

/// stop a bot with the given name
#[poise::command(slash_command)]
async fn terminate(
    ctx: Context<'_>,
    #[description = "Id of the task of interest"]
    #[autocomplete = "autocomplete_taskid"]
    taskid: String,
) -> Result<(), Error> {
    let candidates = autocomplete_taskid(ctx, "").await;
    if candidates.iter().any(|name| *name == taskid) {
        let interaction = slash_ctx_as_responsibe_interaction(&ctx);
        if !auth::authenticate(
            ctx.serenity_context(),
            &interaction,
            &format!("bothub_terminate_{taskid}"),
        )
        .await?
        {
            interaction
                .create_followup_message(&ctx, |msg| msg.ephemeral(true).content("Nope!\n"))
                .await?;
            return Ok(());
        }

        let stdio_lock = match timeout(
            tokio::time::Duration::from_secs(30),
            ctx.data().stdio_lock.lock(),
        )
        .await
        {
            Ok(lock) => lock,
            Err(_) => {
                interaction
                    .create_followup_message(ctx, |m| {
                        m.ephemeral(true).content("Failed sending command to bothub.")
                    })
                    .await?;
                return Ok(());
            }
        };

        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        let command_result = {
            println!("terminate {}", taskid);
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

        interaction
            .create_followup_message(ctx, |m| {
                m.ephemeral(true).content(format!("Result:\n```\n{}```", command_result))
            })
            .await?;
    } else {
        ctx.say(format!(
            "Task\n```\n{}\n```isn't found in task list and is filtered for security reasons.",
            taskid
        ))
        .await?;
    }

    Ok(())
}

/// kill the control bot (hey that's me), then attempt to restart it
#[poise::command(slash_command, prefix_command, rename = "control-restart")]
async fn control_restart(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|msg| {
        msg.content("Click to RESTART the bot")
            .ephemeral(true)
            .components(|comp| {
                comp.create_action_row(|row| {
                    row.create_button(|btn| {
                        btn.custom_id("ctrl_restart.btn")
                            .style(serenity::ButtonStyle::Danger)
                            .label("RESTART")
                    })
                })
            })
    })
    .await?;
    Ok(())
}

pub async fn ctrl_restart_btn_handler<'a>(
    ctx: &serenity::Context,
    interaction: &ResponsibleInteraction<'a>,
) -> Result<(), Error> {
    if !auth::authenticate(&ctx, &interaction, "ctrl_restart").await? {
        interaction
            .create_followup_message(&ctx, |msg| {
                msg.ephemeral(true)
                    .content("Don't even try to stop me lol\n")
            })
            .await?;
        Ok(())
    } else {
        interaction
            .create_followup_message(&ctx, |msg| {
                msg.ephemeral(true)
                    .content("Restarting the bot in 5 seconds...")
            })
            .await?;
        eprintln!("Restarting the bot in 5 seconds...");
        eprintln!("Triggered by {}", interaction.user());
        ctx.set_presence(None, serenity::OnlineStatus::DoNotDisturb)
            .await;
        tokio::time::sleep(Duration::from_secs(5)).await;
        eprintln!("Stopping the bot...");
        match ctx.data.read().await.get::<ShardManagerContainer>() {
            Some(v) => v,
            None => {
                interaction
                    .create_followup_message(&ctx, |msg| {
                        msg.ephemeral(true).content("Failed stopping the bot...")
                    })
                    .await?;
                eprintln!("Failed stopping the bot...");
                return Ok(());
            }
        }
        .lock()
        .await
        .shutdown_all()
        .await;
        println!("control-restart");
        Ok(())
    }
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

    bot_list
        .split(' ')
        .filter(|name| name.starts_with(partial))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

async fn autocomplete_ebotname<'a>(ctx: Context<'_>, partial: &'a str) -> Vec<String> {
    let bot_list = if let Ok(stdio_lock) = timeout(
        tokio::time::Duration::from_secs(30),
        ctx.data().stdio_lock.lock(),
    )
    .await
    {
        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        println!("list-existing");
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

    bot_list
        .split(' ')
        .filter(|name| name.starts_with(partial))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}

async fn autocomplete_taskid<'a>(ctx: Context<'_>, partial: &'a str) -> Vec<String> {
    let task_list = if let Ok(stdio_lock) = timeout(
        tokio::time::Duration::from_secs(30),
        ctx.data().stdio_lock.lock(),
    )
    .await
    {
        let mut stdin_reader = ctx.data().stdin_linereader.lock().await;
        println!("list-executing");
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

    task_list
        .split(' ')
        .filter(|name| name.starts_with(partial))
        .map(|name| name.to_string())
        .collect::<Vec<String>>()
}
