use crate::{Context, Error};
use poise::builtins::register_application_commands_buttons;

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    // Handles the registration for us.
    register_application_commands_buttons(ctx).await?;

    Ok(())
}
