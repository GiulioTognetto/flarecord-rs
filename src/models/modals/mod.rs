use std::sync::Arc;

use dynosaur::dynosaur;
use twilight_model::{
    application::interaction::InteractionContextType,
    guild::Permissions,
    oauth::ApplicationIntegrationType,
};

use crate::{
    error::{BotResult, Error},
    models::{
        command::response::CommandResponse,
        context::InteractionContext,
        modals::interaction::ModalInteraction,
    },
};

pub mod data;
pub mod interaction;
pub mod root;

pub use crate::models::components::modal::{
    FileUpload, Label, ModalSelect, TextDisplay, TextInput, TextInputStyle, TextStyle,
};
pub use root::{ModalComponent, RootModal};

pub type ModalType = Arc<DynModal<'static>>;

#[allow(async_fn_in_trait)]
#[dynosaur(DynModal = dyn(box) Modal)]
pub trait Modal: Send + Sync {
    fn name(&self) -> String;
    fn description(&self) -> String;

    fn default_member_permissions(&self) -> Option<Permissions> {
        None
    }

    fn interaction_contexts(&self) -> Vec<InteractionContextType> {
        vec![]
    }

    fn integration_types(&self) -> Vec<ApplicationIntegrationType> {
        vec![]
    }

    fn id(&self) -> String {
        self.name()
    }

    fn title(&self) -> String {
        self.description()
    }

    fn build(&self, _root: &mut RootModal) {}

    async fn on_submit(
        &self,
        _interaction: ModalInteraction,
        _ctx: InteractionContext,
    ) -> BotResult<CommandResponse> {
        Err(Error::ExecuteNotImplemented(self.name()))
    }
}

pub trait IntoModal {
    fn into_modal(self) -> ModalType;
}

impl<M: Modal + 'static> IntoModal for M {
    fn into_modal(self) -> ModalType {
        DynModal::new_arc(self)
    }
}
