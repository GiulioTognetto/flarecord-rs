use twilight_model::channel::message::Component;

use crate::{
    models::modals::root::ModalComponent,
    traits::component::IntoTwilight,
};

pub struct Label {
    label: String,
    description: Option<String>,
    component: Box<ModalComponent>,
}

impl Label {
    pub fn new(label: impl Into<String>, component: impl Into<ModalComponent>) -> Self {
        Self {
            label: label.into(),
            description: None,
            component: Box::new(component.into()),
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl IntoTwilight<Component> for Label {
    fn into_twilight(self) -> Component {
        Component::Label(twilight_model::channel::message::component::Label {
            id: None,
            label: self.label,
            description: self.description,
            component: Box::new((*self.component).into()),
        })
    }
}
