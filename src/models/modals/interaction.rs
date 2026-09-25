use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

use twilight_model::{application::{interaction::{InteractionContextType, InteractionData, InteractionPartialGuild}, monetization::Entitlement}, channel::Channel, guild::{PartialMember, Permissions}, id::{AnonymizableId, Id, marker::{ApplicationMarker, ChannelMarker, GuildMarker, InteractionMarker, UserMarker}}, oauth::ApplicationIntegrationMap};

use crate::{
    error::{BotResult, Error},
    models::{
        command::response::CommandResponse,
        interaction::Interaction,
        modals::data::ModalData,
        user::{User, UserRef},
    },
    services::discord::DISCORD_SERVICE,
};

#[allow(unused)]
pub struct ModalInteraction {
    pub data: ModalData, 
    pub id: Id<InteractionMarker>,
    pub locale: String,
    pub token: String,
    
    pub channel: Option<Channel>,
    pub channel_id: Option<Id<ChannelMarker>>,
    
    pub guild: Option<InteractionPartialGuild>,
    pub guild_id: Option<Id<GuildMarker>>,
    pub guild_locale: Option<String>,
    
    pub member: Option<PartialMember>,
    pub user: Option<User>,
    
    pub context: Option<InteractionContextType>,

    pub entitlements: Vec<Entitlement>,
    pub app_permissions: Option<Permissions>,
    pub application_id: Id<ApplicationMarker>,
    pub authorizing_integration_owners: ApplicationIntegrationMap<AnonymizableId<GuildMarker>, Id<UserMarker>>,
    response_sent: Arc<AtomicBool>,
}

#[allow(unused)]
impl ModalInteraction {
    pub fn author<'a>(&'a self) -> Option<UserRef<'a>> {
        match self.member.as_ref() {
            Some(member) if member.user.is_some() => member.user.as_ref().map(|a| a.into()),
            _ => self.user.as_ref().map(|a| a.into()),
        }
    }

    pub fn author_id(&self) -> Option<Id<UserMarker>> {
        self.author().map(|a| a.id)
    }

    pub async fn defer(&self, ephemeral: bool) -> BotResult<()> {
        let service = DISCORD_SERVICE.get().expect("DiscordService should be Some");
        service.defer(self.id, &self.token, false, ephemeral).await?;
        self.response_sent.store(true, Ordering::Release);
        Ok(())
    }

    pub async fn edit(&self, response: CommandResponse) -> BotResult<CommandResponse> {
        let service = DISCORD_SERVICE.get().expect("DiscordService should be Some");
        service.edit(self.application_id, &self.token, response).await?;
        self.response_sent.store(true, Ordering::Release);
        Ok(CommandResponse::new())
    }

    pub(crate) fn response_state(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.response_sent)
    }
}

impl TryFrom<Interaction> for ModalInteraction {
    type Error = Error;

    fn try_from(mut value: Interaction) -> Result<Self, Self::Error> {
        let data = match value.data.take() {
            Some(InteractionData::ModalSubmit(d)) => ModalData::from(*d),
            _ => return Err(Error::Generic("Expected ModalSubmit interaction".into())),
        };

        Ok(Self {
            application_id: value.application_id,
            authorizing_integration_owners: value.authorizing_integration_owners.clone(),
            channel: value.channel.take(),
            context: value.context.take(),
            entitlements: std::mem::take(&mut value.entitlements),
            guild: value.guild.take(),
            guild_locale: value.guild_locale.take(),
            locale: value.locale.take().unwrap_or_else(|| "en-US".into()),
            data: data,
            id: value.id,
            token: std::mem::take(&mut value.token),
            #[allow(deprecated)]
            channel_id: value.channel_id,
            guild_id: value.guild_id,
            member: value.member.take(),
            user: value.user.take().map(|u| u.into()),
            app_permissions: value.app_permissions,
            response_sent: Arc::new(AtomicBool::new(false)),
        })
    }
}