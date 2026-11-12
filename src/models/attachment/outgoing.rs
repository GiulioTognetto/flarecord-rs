use twilight_model::http::attachment::Attachment as TwilightOutgoingAttachment;

use crate::traits::component::IntoTwilight;

#[allow(unused)]
pub struct Attachment(TwilightOutgoingAttachment);

#[allow(unused)]
impl Attachment {
    pub fn new(filename: impl Into<String>) -> Self {
        Self(TwilightOutgoingAttachment {
            title: None,
            filename: filename.into(),
            duration_secs: None,
            description: None,
            is_spoiler: None,
            waveform: None,
            file: vec![],
            id: 0
        })
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.0.title = Some(title.into());
    }

    pub fn set_description(&mut self, description: impl Into<String>) {
        self.0.description = Some(description.into());
    }

    pub fn set_content(&mut self, content: impl Into<Vec<u8>>) {
        self.0.file = content.into()
    }

    pub fn set_waveform(&mut self, waveform: String) {
        self.0.waveform = Some(waveform)
    }

    pub fn set_duration_secs(&mut self, duration: f64) {
        self.0.duration_secs = Some(duration)
    }

    pub fn set_is_spoiler(&mut self, is_spoiler: bool) {
        self.0.is_spoiler = Some(is_spoiler)
    }

    pub (crate) fn set_id(&mut self, id: u64) {
        self.0.id = id
    }
}

impl Default for Attachment {
    fn default() -> Self {
        Self::new("empty-file")
    }
}

impl IntoTwilight<TwilightOutgoingAttachment> for Attachment {
    fn into_twilight(self) -> TwilightOutgoingAttachment {
        self.0
    }
}

impl From<TwilightOutgoingAttachment> for Attachment {
    fn from(value: TwilightOutgoingAttachment) -> Self {
        Self(value)
    }
}