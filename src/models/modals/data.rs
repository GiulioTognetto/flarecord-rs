use std::ops::Deref;

use twilight_model::application::interaction::modal::{
    ModalInteractionComponent,
    ModalInteractionData as TwilightModalData,
};

pub struct ModalData(pub(crate) TwilightModalData);

impl ModalData {
    pub fn custom_id(&self) -> &str {
        &self.0.custom_id
    }

    pub fn value(&self, custom_id: &str) -> Option<&str> {
        fn find<'a>(
            components: &'a [ModalInteractionComponent],
            custom_id: &str,
        ) -> Option<&'a str> {
            for component in components {
                match component {
                    ModalInteractionComponent::TextInput(input)
                        if input.custom_id == custom_id =>
                    {
                        return Some(input.value.as_str());
                    }
                    ModalInteractionComponent::ActionRow(row) => {
                        if let Some(value) = find(&row.components, custom_id) {
                            return Some(value);
                        }
                    }
                    _ => continue,
                }
            }

            None
        }

        find(&self.0.components, custom_id)
    }

    pub fn text(&self, custom_id: &str) -> Option<&str> { self.value(custom_id) }

    pub fn values(&self, custom_id: &str) -> Option<Vec<String>> {
        fn find(components: &[ModalInteractionComponent], custom_id: &str) -> Option<Vec<String>> {
            for component in components {
                match component {
                    ModalInteractionComponent::StringSelect(s) if s.custom_id == custom_id => return Some(s.values.clone()),
                    ModalInteractionComponent::UserSelect(s) if s.custom_id == custom_id => return Some(s.values.iter().map(ToString::to_string).collect()),
                    ModalInteractionComponent::RoleSelect(s) if s.custom_id == custom_id => return Some(s.values.iter().map(ToString::to_string).collect()),
                    ModalInteractionComponent::MentionableSelect(s) if s.custom_id == custom_id => return Some(s.values.iter().map(ToString::to_string).collect()),
                    ModalInteractionComponent::ChannelSelect(s) if s.custom_id == custom_id => return Some(s.values.iter().map(ToString::to_string).collect()),
                    ModalInteractionComponent::Label(l) => if let Some(v) = find(std::slice::from_ref(l.component.as_ref()), custom_id) { return Some(v); },
                    ModalInteractionComponent::ActionRow(r) => if let Some(v) = find(&r.components, custom_id) { return Some(v); },
                    _ => {}
                }
            }
            None
        }
        find(&self.0.components, custom_id)
    }

    pub fn select_values(&self, custom_id: &str) -> Option<Vec<String>> { self.values(custom_id) }
}

impl From<TwilightModalData> for ModalData {
    fn from(value: TwilightModalData) -> Self {
        Self(value)
    }
}

impl Deref for ModalData {
    type Target = TwilightModalData;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
