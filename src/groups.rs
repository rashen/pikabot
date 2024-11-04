use crate::{Context, Error};
use anyhow::anyhow;
use poise::serenity_prelude::{EditRole, Permissions, Role};

#[poise::command(
    prefix_command,
    slash_command,
    subcommands("list", "join", "leave", "create")
)]
pub async fn groups(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

fn get_cosmetic_roles(ctx: &Context<'_>) -> Vec<Role> {
    let cosmetic = Permissions::from_bits_truncate(0);
    match ctx.guild() {
        None => vec![],
        Some(g) => g
            .roles
            .iter()
            .filter_map(|(_, v)| match v.permissions == cosmetic {
                true => Some(v.clone()),
                false => None,
            })
            .collect::<Vec<Role>>(),
    }
}

#[poise::command(prefix_command, slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let mut roles = get_cosmetic_roles(&ctx);
    roles.sort();
    let msg = match roles.is_empty() {
        true => String::from("No groups found"),
        false => roles
            .iter()
            .map(|r| {
                let name = &r.name;
                format!("`{name}`")
            })
            .intersperse(", ".to_string())
            .collect::<String>(),
    };
    ctx.say(msg).await?;

    Ok(())
}

async fn autocomplete_group(ctx: Context<'_>, partial: &str) -> Vec<String> {
    get_cosmetic_roles(&ctx)
        .iter()
        .filter_map(|r| match r.name.contains(partial) {
            true => Some(r.name.clone()),
            false => None,
        })
        .collect::<Vec<String>>()
}

#[poise::command(prefix_command, slash_command)]
pub async fn join(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_group"] group: String,
) -> Result<(), Error> {
    let roles = get_cosmetic_roles(&ctx);

    if let Some(role) = roles.iter().find(|r| r.name == group) {
        if let Some(guild) = ctx.guild().and_then(|g| Some(g.id)) {
            let member = guild.member(ctx.http(), ctx.author()).await?;
            member.add_role(ctx.http(), role.id).await?;
            let member_name = member.display_name();
            let role_name = role.name.as_str();
            ctx.say(format!("{member_name} joined the group `{role_name}`",))
                .await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn leave(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_group"] group: String,
) -> Result<(), Error> {
    let roles = get_cosmetic_roles(&ctx);

    if let Some(role) = roles.iter().find(|r| r.name == group) {
        if let Some(guild) = ctx.guild().and_then(|g| Some(g.id)) {
            let member = guild.member(ctx.http(), ctx.author()).await?;
            member.remove_role(ctx.http(), role.id).await?;
            let member_name = member.display_name();
            let role_name = role.name.as_str();
            ctx.say(format!("{member_name} left group `{role_name}`",))
                .await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn create(ctx: Context<'_>, group: String) -> Result<(), Error> {
    let member = ctx.author_member().await;
    let Some(member) = member else {
        return Err(anyhow!("Could not get member"));
    };

    let Some(new_role) = get_cosmetic_roles(&ctx).first().and_then(|r: &Role| {
        let name = group.clone();
        Some(EditRole::from_role(r).name(name))
    }) else {
        return Err(anyhow!("Failed to get already existing roles"));
    };

    let Some(guild) = ctx.guild_id() else {
        return Err(anyhow!("Failed to fetch guilds"));
    };

    let Ok(role) = guild.create_role(ctx.http(), new_role).await else {
        return Err(anyhow!("Failed to create role"));
    };

    let _ = member.add_role(ctx.http(), role.id).await;
    let author = member.display_name().to_string();
    ctx.say(format!(
        "{author} created the new group {group}. Use `/groups join {group}` to join.",
    ))
    .await?;

    Ok(())
}
