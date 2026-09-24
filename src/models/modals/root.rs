use twilight_model::channel::message::{Component, component::{Label, SelectMenu}};
use super::input::TextInput;
use crate::{
    models::components::interactive::select::Select,
    traits::component::IntoTwilight,
};

pub enum ModalComponent {
    TextInput(TextInput),
    Select(SelectMenu),
    Label { label: String, description: Option<String>, component: Box<ModalComponent> },
}

pub struct RootModal {
    pub(crate) components: Vec<ModalComponent>,
}

impl RootModal {
    pub fn new() -> Self {
        Self { components: Vec::new() }
    }

    pub fn add(&mut self, component: impl Into<ModalComponent>) {
        self.components.push(component.into());
    }

    pub(crate) fn into_components(self) -> Vec<Component> {
        self.components.into_iter().map(Into::into).collect()
    }
}

impl From<TextInput> for ModalComponent {
    fn from(value: TextInput) -> Self { Self::TextInput(value) }
}

impl From<SelectMenu> for ModalComponent {
    fn from(value: SelectMenu) -> Self { Self::Select(value) }
}

impl From<Select> for ModalComponent {
    fn from(value: Select) -> Self {
        Self::Select(value.into_twilight())
    }
}

impl From<ModalComponent> for Component {
    fn from(value: ModalComponent) -> Self {
        match value {
            ModalComponent::TextInput(input) => Component::TextInput(input.into_twilight()),
            ModalComponent::Select(select) => Component::SelectMenu(select),
            ModalComponent::Label { label, description, component } => Component::Label(Label {
                id: None, label, description, component: Box::new((*component).into()),
            }),
        }
    }
}

impl Default for RootModal {
    fn default() -> Self {
        Self::new()
    }
}
