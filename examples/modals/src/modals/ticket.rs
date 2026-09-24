use flarecord::prelude::*;
use twilight_model::channel::message::component::SelectMenuOption;

pub struct TicketModal {
}

impl Modal for TicketModal {
    fn name(&self) -> String {
        "ticket-modal".into()
    }

    fn description(&self) -> String {
        "a modal used to send a ticket".into()
    }

    fn build(&self, root: &mut RootModal) {
        root.add(
            TextInput::new("subject", "Subject")
                .required(true),
        );
        root.add(
            TextInput::new("description", "Description")
                .style(TextInputStyle::Paragraph)
                .required(true),
        );
        root.add(
            Select::string()
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
                .build(),
        );
    }

    async fn on_submit(&self, interaction: ModalInteraction, _ctx: InteractionContext) -> BotResult<CommandResponse> {
        let subject = interaction.data.text("subject").unwrap_or_default();
        let description = interaction.data.text("description").unwrap_or_default();
        let priority = interaction.data.select_values("priority")
            .map(|values| values.join(", ")).unwrap_or_else(|| "normal".into());
        Ok(CommandResponse::builder()
            .content(format!("Ticket received ({priority}): {subject}\n{description}"))
            .build())
    }
}