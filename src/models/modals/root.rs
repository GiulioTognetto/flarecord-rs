use twilight_model::channel::message::Component;

use crate::{
    models::components::modal::{FileUpload, Label, ModalSelect, TextDisplay, TextInput},
    traits::component::IntoTwilight,
};

pub enum ModalComponent {
    TextInput(TextInput),
    Select(ModalSelect),
    Label(Label),
    TextDisplay(TextDisplay),
    FileUpload(FileUpload),
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

impl From<ModalSelect> for ModalComponent {
    fn from(value: ModalSelect) -> Self { Self::Select(value) }
}

impl From<Label> for ModalComponent {
    fn from(value: Label) -> Self { Self::Label(value) }
}

impl From<TextDisplay> for ModalComponent {
    fn from(value: TextDisplay) -> Self { Self::TextDisplay(value) }
}

impl From<FileUpload> for ModalComponent {
    fn from(value: FileUpload) -> Self { Self::FileUpload(value) }
}

impl From<ModalComponent> for Component {
    fn from(value: ModalComponent) -> Self {
        match value {
            ModalComponent::TextInput(input) => Component::TextInput(input.into_twilight()),
            ModalComponent::Select(select) => Component::SelectMenu(select.into_twilight()),
            ModalComponent::Label(label) => label.into_twilight(),
            ModalComponent::TextDisplay(text_display) => {
                Component::TextDisplay(text_display.into_twilight())
            }
            ModalComponent::FileUpload(file_upload) => {
                Component::FileUpload(file_upload.into_twilight())
            }
        }
    }
}

impl Default for RootModal {
    fn default() -> Self {
        Self::new()
    }
}
