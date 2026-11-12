use twilight_model::channel::message::embed::{EmbedMediaFlags, EmbedVideo as TwilightEmbedVideo};


pub struct EmbedVideo(TwilightEmbedVideo);

impl Default for EmbedVideo {
    fn default() -> Self {
        Self::new()
    }
}

impl EmbedVideo {
    pub fn new() -> Self {
        Self(TwilightEmbedVideo {
            height: None,
            width: None,
            proxy_url: None,
            url: None,
            content_type: None,
            placeholder: None,
            placeholder_version: None,
            flags: None
        })
    }

    pub fn set_url<IntoString: Into<String>>(&mut self, url: impl Into<Option<String>>) {
        self.0.url = url.into()
    }

    pub fn set_height(&mut self, height: impl Into<Option<u64>>) {
        self.0.height = height.into()
    }

    pub fn set_width(&mut self, width: impl Into<Option<u64>>) {
        self.0.height = width.into()
    }
    pub fn set_placeholder(&mut self, placeholder: impl Into<Option<String>>) {
        self.0.placeholder = placeholder.into()
    }

    pub fn set_placeholder_version(&mut self, placeholder_version: impl Into<Option<u64>>) {
        self.0.placeholder_version = placeholder_version.into()
    }

    pub fn set_content_type(&mut self, content_type: impl Into<Option<String>>) {
        self.0.content_type = content_type.into()
    }

    pub fn set_flags(&mut self, flags: EmbedMediaFlags) {
        self.0.flags = Some(flags);
    }
}

impl From<TwilightEmbedVideo> for EmbedVideo {
    fn from(value: TwilightEmbedVideo) -> Self {
        Self(value)
    }
}

impl From<EmbedVideo> for TwilightEmbedVideo {
    fn from(val: EmbedVideo) -> Self {
        val.0
    }
}