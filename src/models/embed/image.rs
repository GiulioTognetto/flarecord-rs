use twilight_model::channel::message::embed::{EmbedImage as TwilightEmbedImage, EmbedMediaFlags};



pub struct EmbedImage(TwilightEmbedImage);

impl EmbedImage {
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightEmbedImage {
            content_type: None,
            placeholder: None,
            placeholder_version: None,
            height: None,
            width: None,
            flags: None,
            proxy_url: None,
            url: url.into()
        })
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.0.url = url.into()
    }

    pub fn set_content_type(&mut self, content_type: Option<String>) {
        self.0.content_type = content_type;
    }

    pub fn set_placeholder(&mut self, placeholder: Option<String>) {
        self.0.placeholder = placeholder;
    }

    pub fn set_placeholder_version(&mut self, placeholder_version: Option<u64>) {
        self.0.placeholder_version = placeholder_version;
    }

    pub fn set_flags(&mut self, flags: EmbedMediaFlags) {
        self.0.flags = Some(flags);
    }

    pub fn set_height(&mut self, height: Option<u64>) {
        self.0.height = height;
    }

    pub fn set_width(&mut self, width: Option<u64>) {
        self.0.height = width;
    }
}

impl From<&str> for EmbedImage {
    fn from(val: &str) -> Self {
        EmbedImage::new(val)
    }
}

impl From<String> for EmbedImage {
    fn from(val: String) -> Self {
        EmbedImage::new(val)
    }
}

impl From<TwilightEmbedImage> for EmbedImage {
    fn from(value: TwilightEmbedImage) -> Self {
        Self(value)
    }
}

impl From<EmbedImage> for TwilightEmbedImage {
    fn from(val: EmbedImage) -> Self {
        val.0
    }
}