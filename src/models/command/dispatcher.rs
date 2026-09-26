use crate::{
    error::{BotResult, Error}, models::{command::{
        Command, 
        CommandType, 
        Subcommand, 
        SubcommandGroup, 
        SubcommandGroupType,
        interaction::CommandInteraction, 
        response::CommandResponse
    }, context::InteractionContext}
};

pub (crate) struct CommandDispatcher;

impl CommandDispatcher {
    pub (crate) async fn dispatch(
        cmd: &CommandType,
        interaction: CommandInteraction,
        ctx: InteractionContext
    ) -> BotResult<CommandResponse> {
        if let Some(group_name) = interaction.data.get_subcommand_group_name()
            && let Some(group) = cmd.groups().iter().find(|g| g.name() == group_name) {
                let Some(inner_interaction) = interaction.with_inner_data() else {
                    return Err(Error::InvalidInteraction("Missing inner data for the subcommand!".to_string()));
                };

                return Self::dispatch_group(group, inner_interaction, ctx).await
            }

        if let Some(sub_name) = interaction.data.get_subcommand_name()
            && let Some(sub) = cmd.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_interaction) = interaction.with_inner_data() else {
                    return Err(Error::InvalidInteraction("Missing inner data for the subcommand!".to_string()));
                };

                return sub.execute(inner_interaction, ctx).await
            }

        cmd.execute(interaction, ctx).await
    }

    async fn dispatch_group(
        group: &SubcommandGroupType,
        interaction: CommandInteraction,
        ctx: InteractionContext
    ) -> BotResult<CommandResponse> {
        if let Some(sub_name) = interaction.data.get_subcommand_name()
            && let Some(sub) = group.subcommands().iter().find(|s| s.name() == sub_name) {
                let Some(inner_interaction) = interaction.with_inner_data() else {
                    return Err(Error::InvalidInteraction("Missing inner data for the subcommand!".to_string()));
                };

                return sub.execute(inner_interaction, ctx).await
            }

        Err(Error::CommandNotFound("Subcommand not found in group".into()))
    }
}