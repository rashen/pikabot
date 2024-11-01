use crate::{Context, Error};
use poise::serenity_prelude::{Permissions, Role};

#[poise::command(prefix_command, slash_command, subcommands("list", "add", "remove"))]
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
    let roles = get_cosmetic_roles(&ctx);
    let msg = match roles.is_empty() {
        true => String::from("No groups found"),
        false => roles
            .iter()
            .fold(String::new(), |acc, r| acc + " " + &r.name),
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
pub async fn add(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_group"] group: String,
) -> Result<(), Error> {
    let roles = get_cosmetic_roles(&ctx);

    if let Some(role) = roles.iter().find(|r| r.name == group) {
        if let Some(guild) = ctx.guild().and_then(|g| Some(g.id)) {
            let member = guild.member(ctx.http(), ctx.author()).await?;
            member.add_role(ctx.http(), role.id).await?;
            ctx.say(format!(
                "Added {} to group {}",
                member.display_name(),
                role.name
            ))
            .await?;
        }
    }

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn remove(ctx: Context<'_>, group: String) -> Result<(), Error> {
    let roles = get_cosmetic_roles(&ctx);

    if let Some(role) = roles.iter().find(|r| r.name == group) {
        if let Some(guild) = ctx.guild().and_then(|g| Some(g.id)) {
            let member = guild.member(ctx.http(), ctx.author()).await?;
            member.remove_role(ctx.http(), role.id).await?;
            ctx.say(format!(
                "Removed {} from group {}",
                member.display_name(),
                role.name
            ))
            .await?;
        }
    }

    Ok(())
}
