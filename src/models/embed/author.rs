use twilight_model::channel::message::embed::EmbedAuthor as TwilightEmbedAuthor;

pub struct EmbedAuthor(TwilightEmbedAuthor);

impl EmbedAuthor {
    pub fn new(name: impl Into<String>) -> Self {
        Self(TwilightEmbedAuthor { 
            name: name.into(),
            icon_url: None,
            proxy_icon_url: None,
            url: None
        })
    }

    pub fn set_icon_url(&mut self, icon_url: Option<String>) {
        self.0.icon_url = icon_url;
    }

    pub fn set_url(&mut self, url: Option<String>) {
        self.0.url = url;
    }
}

impl From<EmbedAuthor> for TwilightEmbedAuthor {
    fn from(val: EmbedAuthor) -> Self {
        val.0
    }
}

impl From<TwilightEmbedAuthor> for EmbedAuthor {
    fn from(val: TwilightEmbedAuthor) -> Self {
        EmbedAuthor(val)
    }
}