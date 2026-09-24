use std::ops::Deref;

use twilight_model::application::interaction::modal::{
    ModalInteractionComponent, ModalInteractionData as TwilightModalData,
};
use twilight_model::{
    application::interaction::{
        InteractionChannel, InteractionDataResolved, InteractionMember,
    },
    channel::{Message, message::component::ComponentType as TwilightComponentType},
    guild::Role,
    id::{
        Id,
        marker::{
            AttachmentMarker, ChannelMarker, MessageMarker, RoleMarker, UserMarker,
        },
    },
};

use crate::{
    models::{attachment::incoming::IncomingAttachmentRef, user::UserRef},
};

pub struct ModalData(pub(crate) TwilightModalData);

impl ModalData {
    pub fn custom_id(&self) -> &str {
        self.0.custom_id.as_str()
    }

    pub fn components(&self) -> &[ModalInteractionComponent] {
        self.0.components.as_slice()
    }

    pub fn get_component(&self, custom_id: &str) -> Option<&ModalInteractionComponent> {
        find_component(&self.0.components, custom_id)
    }

    pub fn get_component_type(&self, custom_id: &str) -> Option<TwilightComponentType> {
        self.get_component(custom_id)
            .map(ModalInteractionComponent::kind)
    }

    pub fn get_text_input(&self, custom_id: &str) -> Option<&str> {
        match self.get_component(custom_id)? {
            ModalInteractionComponent::TextInput(input) => Some(input.value.as_str()),
            _ => None,
        }
    }

    pub fn get_select_values(&self, custom_id: &str) -> Option<Vec<String>> {
        match self.get_component(custom_id)? {
            ModalInteractionComponent::StringSelect(select) => Some(select.values.clone()),
            ModalInteractionComponent::UserSelect(select) => {
                Some(select.values.iter().map(ToString::to_string).collect())
            }
            ModalInteractionComponent::RoleSelect(select) => {
                Some(select.values.iter().map(ToString::to_string).collect())
            }
            ModalInteractionComponent::MentionableSelect(select) => {
                Some(select.values.iter().map(ToString::to_string).collect())
            }
            ModalInteractionComponent::ChannelSelect(select) => {
                Some(select.values.iter().map(ToString::to_string).collect())
            }
            _ => None,
        }
    }

    pub fn get_file_upload_values(
        &self,
        custom_id: &str,
    ) -> Option<&[Id<AttachmentMarker>]> {
        match self.get_component(custom_id)? {
            ModalInteractionComponent::FileUpload(file_upload) => {
                Some(file_upload.values.as_slice())
            }
            _ => None,
        }
    }

    pub fn resolved(&self) -> Option<&InteractionDataResolved> {
        self.0.resolved.as_ref()
    }

    pub fn get_resolved_user<'a>(&'a self, id: Id<UserMarker>) -> Option<UserRef<'a>> {
        self.0.resolved.as_ref()?.users.get(&id).map(UserRef::from)
    }

    pub fn get_resolved_member<'a>(&'a self, id: Id<UserMarker>) -> Option<&'a InteractionMember> {
        self.0.resolved.as_ref()?.members.get(&id)
    }

    pub fn get_resolved_role<'a>(&'a self, id: Id<RoleMarker>) -> Option<&'a Role> {
        self.0.resolved.as_ref()?.roles.get(&id)
    }

    pub fn get_resolved_message<'a>(&'a self, id: Id<MessageMarker>) -> Option<&'a Message> {
        self.0.resolved.as_ref()?.messages.get(&id.cast())
    }

    pub fn get_resolved_channel<'a>(
        &'a self,
        id: Id<ChannelMarker>,
    ) -> Option<&'a InteractionChannel> {
        self.0.resolved.as_ref()?.channels.get(&id)
    }

    pub fn get_resolved_attachment<'a>(
        &'a self,
        id: Id<AttachmentMarker>,
    ) -> Option<IncomingAttachmentRef<'a>> {
        self.0
            .resolved
            .as_ref()?
            .attachments
            .get(&id)
            .map(Into::into)
    }
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

fn find_component<'a>(
    components: &'a [ModalInteractionComponent],
    custom_id: &str,
) -> Option<&'a ModalInteractionComponent> {
    for component in components {
        let found = match component {
            ModalInteractionComponent::TextInput(input)
                if input.custom_id == custom_id =>
            {
                Some(component)
            }
            ModalInteractionComponent::StringSelect(select)
                if select.custom_id == custom_id =>
            {
                Some(component)
            }
            ModalInteractionComponent::UserSelect(select)
                if select.custom_id == custom_id =>
            {
                Some(component)
            }
            ModalInteractionComponent::RoleSelect(select)
                if select.custom_id == custom_id =>
            {
                Some(component)
            }
            ModalInteractionComponent::MentionableSelect(select)
                if select.custom_id == custom_id =>
            {
                Some(component)
            }
            ModalInteractionComponent::ChannelSelect(select)
                if select.custom_id == custom_id =>
            {
                Some(component)
            }
            ModalInteractionComponent::Label(label) => {
                find_component(std::slice::from_ref(label.component.as_ref()), custom_id)
            }
            ModalInteractionComponent::ActionRow(row) => {
                find_component(&row.components, custom_id)
            }
            _ => None,
        };

        if found.is_some() {
            return found;
        }
    }

    None
}
