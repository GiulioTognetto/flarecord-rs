use twilight_model::channel::{
    ChannelType,
    message::component::{
        SelectMenu as TwilightSelectMenu, SelectMenuOption, SelectMenuType,
    },
};

use crate::traits::component::IntoTwilight;

pub struct ModalSelect {
    pub(crate) inner: TwilightSelectMenu,
}

impl ModalSelect {
    pub fn string() -> Self { Self::new(SelectMenuType::Text) }
    pub fn user() -> Self { Self::new(SelectMenuType::User) }
    pub fn role() -> Self { Self::new(SelectMenuType::Role) }
    pub fn mentionable() -> Self { Self::new(SelectMenuType::Mentionable) }
    pub fn channel() -> Self { Self::new(SelectMenuType::Channel) }

    fn new(kind: SelectMenuType) -> Self {
        let options = match &kind {
            SelectMenuType::Text => Some(Vec::new()),
            _ => None,
        };

        Self {
            inner: TwilightSelectMenu {
                id: None,
                channel_types: None,
                custom_id: String::new(),
                default_values: None,
                disabled: false,
                kind,
                max_values: None,
                min_values: None,
                options,
                placeholder: None,
                required: None,
            },
        }
    }

    pub fn custom_id(mut self, custom_id: impl Into<String>) -> Self {
        self.inner.custom_id = custom_id.into();
        self
    }

    pub fn option(mut self, option: SelectMenuOption) -> Self {
        self.inner.options.get_or_insert_default().push(option);
        self
    }

    pub fn options(mut self, options: Vec<SelectMenuOption>) -> Self {
        self.inner.options = Some(options);
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.inner.placeholder = Some(text.into());
        self
    }

    pub fn min_values(mut self, min: u8) -> Self {
        self.inner.min_values = Some(min);
        self
    }

    pub fn max_values(mut self, max: u8) -> Self {
        self.inner.max_values = Some(max);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.inner.required = Some(required);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.inner.disabled = disabled;
        self
    }

    pub fn channel_types(mut self, channel_types: Vec<ChannelType>) -> Self {
        self.inner.channel_types = Some(channel_types);
        self
    }

    pub fn build(self) -> Self { self }
}

impl IntoTwilight<TwilightSelectMenu> for ModalSelect {
    fn into_twilight(self) -> TwilightSelectMenu { self.inner }
}
