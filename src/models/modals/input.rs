use twilight_model::channel::message::component::{
    TextInput as TwilightTextInput, TextInputStyle as TwilightTextInputStyle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextInputStyle {
    Short,
    Paragraph,
}

impl From<TextInputStyle> for TwilightTextInputStyle {
    fn from(value: TextInputStyle) -> Self {
        match value {
            TextInputStyle::Short => Self::Short,
            TextInputStyle::Paragraph => Self::Paragraph,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextInput {
    custom_id: String,
    label: String,
    style: TextInputStyle,
    placeholder: Option<String>,
    value: Option<String>,
    min_length: Option<u16>,
    max_length: Option<u16>,
    required: bool,
}

impl TextInput {
    pub fn new(custom_id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            custom_id: custom_id.into(),
            label: label.into(),
            style: TextInputStyle::Short,
            placeholder: None,
            value: None,
            min_length: None,
            max_length: None,
            required: true,
        }
    }

    pub fn try_new(custom_id: impl Into<String>, label: impl Into<String>) -> Result<Self, String> {
            let custom_id = custom_id.into();
            let label = label.into();
            if custom_id.is_empty() || custom_id.len() > 100 {
                return Err("modal text input custom_id must contain 1..=100 characters".into());
            }
            if label.is_empty() || label.chars().count() > 45 {
                return Err("modal text input label must contain 1..=45 characters".into());
            }
            Ok(Self::new(custom_id, label))
    }

    pub fn style(mut self, style: TextInputStyle) -> Self {
        self.style = style;
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn min_length(mut self, min_length: u16) -> Self {
        self.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: u16) -> Self {
        self.max_length = Some(max_length);
        self
    }

    pub fn try_min_length(self, min_length: u16) -> Result<Self, String> {
        if min_length > 4000 { return Err("min_length must be <= 4000".into()); }
        if self.max_length.is_some_and(|max| min_length > max) {
            return Err("min_length cannot exceed max_length".into());
        }
        Ok(self.min_length(min_length))
    }

    pub fn try_max_length(self, max_length: u16) -> Result<Self, String> {
        if !(1..=4000).contains(&max_length) { return Err("max_length must be between 1 and 4000".into()); }
        if self.min_length.is_some_and(|min| min > max_length) {
            return Err("max_length cannot be less than min_length".into());
        }
        Ok(self.max_length(max_length))
    }

    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }

    pub(crate) fn into_twilight(self) -> TwilightTextInput {
        TwilightTextInput {
            id: None,
            custom_id: self.custom_id,
            #[allow(deprecated)]
            label: Some(self.label),
            max_length: self.max_length,
            min_length: self.min_length,
            placeholder: self.placeholder,
            required: Some(self.required),
            style: self.style.into(),
            value: self.value,
        }
    }
}
