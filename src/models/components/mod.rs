use std::sync::{Arc, Mutex};

use dynosaur::dynosaur;

use crate::error::BotResult;
use crate::models::command::response::CommandResponse;
use crate::models::components::interaction::ComponentInteraction;
use crate::models::components::layout::LayoutComponent;
use crate::models::context::InteractionContext;
use crate::traits::component::IntoComponent;

pub(crate) mod dispatcher;
pub(crate) mod id;
pub mod interaction;
pub mod content;
pub mod data;
pub mod root;
pub mod layout;
pub mod interactive;

pub use root::RootComponent;

pub type ComponentType = Arc<DynComponent<'static>>;

impl<C: Component + 'static> IntoComponent for C {
    fn into_component(self) -> ComponentType {
        DynComponent::new_arc(self)
    }
}

impl IntoComponent for LayoutComponent {
    fn into_component(self) -> ComponentType {
        let layout_handler = LayoutComponentHandler::new(self);
        DynComponent::new_arc(layout_handler)
    }
}

pub(crate) struct LayoutComponentHandler(Mutex<Option<LayoutComponent>>);

impl LayoutComponentHandler {
    pub fn new(layout: LayoutComponent) -> Self {
        Self(Mutex::new(Some(layout)))
    }
}

impl Component for LayoutComponentHandler {
    fn build(&self, root: &mut RootComponent) {
        if let Ok(mut lock) = self.0.lock() {
            if let Some(layout) = lock.take() {
                root.add(layout);
            }
        }
    }

    async fn handle(
        &self,
        _interaction: ComponentInteraction,
        _ctx: InteractionContext,
    ) -> BotResult<CommandResponse> {
        Ok(CommandResponse::empty())
    }
}

#[allow(async_fn_in_trait)]
#[dynosaur(DynComponent = dyn(box) Component)]
pub trait Component: Send + Sync {
    fn build(&self, root: &mut RootComponent);

    async fn handle(
        &self,
        _interaction: ComponentInteraction,
        _ctx: InteractionContext,
    ) -> BotResult<CommandResponse> {
        Ok(CommandResponse::empty())
    }
}
