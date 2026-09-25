use flarecord::{models::components::content::text_display::TextDisplay, prelude::*};
use twilight_model::channel::message::component::SelectMenuOption;

pub struct TicketModal;

impl Modal for TicketModal {
    fn name(&self) -> String {
        "ticket-modal".into()
    }

    fn description(&self) -> String {
        "a modal used to send a ticket".into()
    }

    fn build(&self, root: &mut RootModal) {
        let subject = TextInput::new("subject", "Subject");
        let description = TextInput::new("description", "Description");

        let proirity = ModalSelect::string()
            .custom_id("priority")
            .placeholder("Select a priority")
                .option(SelectMenuOption {
                    label: "Normal".into(),
                    value: "normal".into(),
                    default: false,
                    description: None,
                    emoji: None,
                })
                .option(SelectMenuOption {
                    label: "High".into(),
                    value: "high".into(),
                    default: false,
                    description: None,
                    emoji: None,
                })
                .option(SelectMenuOption {
                    label: "Urgent".into(),
                    value: "urgent".into(),
                    default: false,
                    description: None,
                    emoji: None,
                })
                .required(true)
                .build();

        root.add(Label::new("Subject", subject));
        root.add(Label::new("Description", description));
        root.add(Label::new("Priority", proirity));
    }

    async fn on_submit(&self, interaction: ModalInteraction, _ctx: InteractionContext) -> BotResult<CommandResponse> {

        Ok(CommandResponse::empty())
    }
}