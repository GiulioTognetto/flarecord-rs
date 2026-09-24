use twilight_model::channel::message::component::FileUpload as TwilightFileUpload;

use crate::traits::component::IntoTwilight;

pub struct FileUpload(TwilightFileUpload);

impl FileUpload {
    pub fn new(custom_id: impl Into<String>) -> Self {
        Self(TwilightFileUpload {
            id: None,
            custom_id: custom_id.into(),
            max_values: None,
            min_values: None,
            required: None,
        })
    }

    pub fn min_values(mut self, min_values: u8) -> Self {
        self.0.min_values = Some(min_values);
        self
    }

    pub fn max_values(mut self, max_values: u8) -> Self {
        self.0.max_values = Some(max_values);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.0.required = Some(required);
        self
    }
}

impl IntoTwilight<TwilightFileUpload> for FileUpload {
    fn into_twilight(self) -> TwilightFileUpload {
        self.0
    }
}
