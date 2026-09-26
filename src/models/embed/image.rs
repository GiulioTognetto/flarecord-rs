use twilight_model::channel::message::embed::EmbedImage as TwilightEmbedImage;



pub struct EmbedImage(TwilightEmbedImage);

impl EmbedImage {
    pub fn new(url: impl Into<String>) -> Self {
        Self(TwilightEmbedImage {
            height: None,
            width: None,
            proxy_url: None,
            url: url.into()
        })
    }

    pub fn set_url(&mut self, url: impl Into<String>) {
        self.0.url = url.into()
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