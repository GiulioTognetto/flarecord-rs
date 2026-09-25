# Modal

Flarecord supporta l'intero ciclo di vita delle modal Discord: apertura tramite `CommandResponse::modal`, parsing di `ModalSubmit` e dispatch al `Modal` registrato.

## Definizione e registrazione

```rust
use flarecord::prelude::*;

struct Ticket;
impl Modal for Ticket {
    fn name(&self) -> String { "ticket".into() }
    fn description(&self) -> String { "Open a ticket".into() }

    fn build(&self, root: &mut RootModal) {
        root.add(Label::new("Subject", TextInput::new("subject", "Subject")));
        root.add(Label::new(
            "Details",
            TextInput::new("details", "Details").style(TextInputStyle::Paragraph),
        ));
        root.add(Label::new("Priority", ModalSelect::string().custom_id("priority")));

    async fn on_submit(&self, interaction: ModalInteraction,
        _ctx: InteractionContext) -> BotResult<CommandResponse> {
        let subject = interaction.data.get_text_input("subject").unwrap_or_default();
        let priorities = interaction.data.get_select_values("priority").unwrap_or_default();
        Ok(CommandResponse::builder()
            .content(format!("{subject}: {priorities:?}"))
            .build())
    }
}

let bot = Bot::builder().register_modal(Ticket).build();
```

`CommandResponse::modal(Ticket)` apre la modal. I componenti sono `TextInput` e i select `string`, `user`, `role`, `mentionable`, `channel` di Twilight. I select possono riusare la stessa configurazione dei message components (`custom_id`, `options`, limiti, placeholder); per una modal usare `root.add_component(...)` e, quando richiesto dall'API Discord, `root.label(...)`/`root.labeled(...)`.

## Dati ricevuti

- `ModalData::custom_id()` identifica la modal.
- `ModalData::text(id)` (alias `value`) legge un TextInput.
- `ModalData::select_values(id)` (alias `values`) restituisce i valori selezionati come `Vec<String>` per ogni tipo di select.
- I componenti racchiusi in `Label` e `ActionRow` sono cercati ricorsivamente.

`TextInput::try_new`, `try_min_length` e `try_max_length` consentono di validare esplicitamente gli input senza panic. Le versioni chainable mantengono compatibilit� con l'API precedente.

Il modal deve essere registrato prima di ricevere il submit; in caso contrario il dispatcher restituisce `ModalNotFound`. L'esempio completo � in `examples/modals`.
