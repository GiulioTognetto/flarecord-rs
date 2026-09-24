use flarecord::prelude::*;



pub struct Ticket;


impl Command for Ticket {
    fn name(&self) -> String {
        "ticket".into()
    }

    fn description(&self) -> String {
        "A command that return a modal".into()
    }

    async fn execute(&self, interaction: CommandInteraction, _ctx: InteractionContext) -> BotResult<CommandResponse> {
        Ok(CommandResponse::modal(crate::modals::ticket::TicketModal {}))
    }
}