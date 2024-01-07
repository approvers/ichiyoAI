use serenity::builder::CreateAttachment;
use serenity::builder::CreateEmbed;
use serenity::builder::CreateInteractionResponse;
use serenity::builder::CreateInteractionResponseFollowup;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::client::{Context, EventHandler};
use serenity::gateway::ActivityData;
use serenity::model::application::CommandDataOptionValue;
use serenity::model::application::CommandType;
use serenity::model::application::{CommandInteraction, Interaction, ResolvedTarget};
use serenity::model::gateway::Ready;
use serenity::model::user::User;

type Result<T> = core::result::Result<T, alloc::borrow::Cow<'static, str>>;

// DaLL-E & ChatGPT で使用するタイムアウト時間の定数
pub const TIMEOUT_DURATION: std::time::Duration = std::time::Duration::from_secs(60);

#[tracing::instrument(skip_all)]
pub async fn is_sponsor(ctx: &Context, user: &User) -> Result<bool> {
    let envs = crate::envs();

    let guild_id = envs.guild_id;
    let role_id = envs.sponsor_role_id;

    user.has_role(ctx, guild_id, role_id)
        .await
        .map_err(|cause| {
            tracing::error!(?cause, "Failed to get sponsor role");
            "Failed to get sponsor role".into()
        })
}

pub struct EvHandler;

enum OptionsField<'a> {
    Model(&'a str),
    Prompt(&'a str),
}

struct Options<'a> {
    model: Option<&'a str>,
    prompt: Option<&'a str>,
}

impl<'a> FromIterator<OptionsField<'a>> for Options<'a> {
    fn from_iter<T: IntoIterator<Item = OptionsField<'a>>>(iter: T) -> Self {
        let mut model = None;
        let mut prompt = None;

        for field in iter {
            match field {
                OptionsField::Model(s) => model = Some(s),
                OptionsField::Prompt(s) => prompt = Some(s),
            }
        }

        Self { model, prompt }
    }
}

#[tracing::instrument(skip_all)]
async fn image(ctx: &Context, ci: &CommandInteraction) -> Result<()> {
    enum Modelname {
        DallE2,
        DallE3,
    }

    let opts = ci
        .data
        .options
        .iter()
        .filter_map(|o| match (&*o.name, &o.value) {
            ("model", CommandDataOptionValue::String(s)) => Some(OptionsField::Model(s)),
            ("prompt", CommandDataOptionValue::String(s)) => Some(OptionsField::Prompt(s)),

            _ => {
                tracing::warn!(?o, "Unexpected option");
                None
            }
        })
        .collect::<Options>();

    let modelname = match opts.model {
        Some("dall-e-2") => Modelname::DallE2,
        Some("dall-e-3") => Modelname::DallE3,

        name => return Err(format!("Unexpected `model`: {}", name.unwrap_or("")).into()),
    };

    let is_sponsor = is_sponsor(ctx, &ci.user).await.unwrap_or(false);
    match (is_sponsor, &modelname) {
        (false, Modelname::DallE3) => {
            return Err("You must be a sponsor to use model \"DALL-E 3\"".into())
        }
        _ => (/* means for all users */),
    }

    let prompt = match opts.prompt {
        Some("") | None => return Err("Unexpected empty `prompt`".into()),
        Some(prompt) => prompt,
    };

    use ichiyo_ai::Generation as _;
    let result = match modelname {
        Modelname::DallE2 => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiDallE2::new(token);

            tokio::time::timeout(TIMEOUT_DURATION, engine.create(prompt)).await
        }
        Modelname::DallE3 => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiDallE3::new(token);

            tokio::time::timeout(TIMEOUT_DURATION, engine.create(prompt)).await
        }
    };

    let ret = match result {
        Ok(result) => result?,
        Err(e) => {
            tracing::error!(?e, "Timeouted to generate image");
            return Err("Timeouted to generate image".into());
        }
    };

    let (content, files) = {
        let (img, meta) = ret;

        let ichiyo_ai::Image { raw, prompt, ext } = img;
        let ichiyo_ai::IMetadata { model } = meta;

        let filename = format!("image.{}", ext.as_str());
        let filedata = raw;

        let content = format!("{model} - **`{prompt}`**");

        (content, [(filename, filedata)])
    };

    let cirf = CreateInteractionResponseFollowup::default()
        .content(content)
        .files(files.map(|(name, raw)| CreateAttachment::bytes(raw, name)));

    ci.create_followup(ctx, cirf).await.map_err(|cause| {
        tracing::error!(?cause, "Failed to create interaction response");
        "Failed to create interaction response"
    })?;

    Ok(())
}

#[tracing::instrument(skip_all)]
async fn completion(ctx: &Context, ci: &CommandInteraction) -> Result<()> {
    enum Modelname {
        GPT35Turbo,
        GPT4Turbo,
        GeminiPro,
    }

    let modelname = match &*ci.data.name {
        "Text (GPT-3.5 Turbo)" => Modelname::GPT35Turbo,
        "Text (GPT-4 Turbo)" => Modelname::GPT4Turbo,
        "Text (Gemini Pro)" => Modelname::GeminiPro,

        name => {
            tracing::error!(?name, "Unexpected command name");
            return Err(format!("Unexpected command name: {name}").into());
        }
    };

    let is_sponsor = is_sponsor(ctx, &ci.user).await.unwrap_or(false);
    match (is_sponsor, &modelname) {
        (false, Modelname::GPT4Turbo) => {
            return Err("You must be a sponsor to use model \"GPT-4 Turbo\"".into())
        }
        _ => (/* means for all users */),
    }

    let Some(ResolvedTarget::Message(msg)) = ci.data.target() else {
        tracing::error!(ci.data.target = ?ci.data.target(), "Unexpected target");
        return Err("Failed to recognize interaction".into());
    };

    // first is newest, last is oldest
    use tokio_stream::StreamExt as _;
    let mut msgs = ChainedMessages::new(ctx.clone(), msg)
        .map(|m| {
            if m.is_own(ctx) {
                // remove metadata if possible
                let content = m
                    .content
                    .rsplit_once("\n\n")
                    .map(|(s, _)| s)
                    .unwrap_or(&m.content)
                    .to_owned();

                ichiyo_ai::Message::Model { content }
            } else {
                let content = m.content.clone();

                ichiyo_ai::Message::User { content }
            }
        })
        .collect::<Vec<_>>()
        .await;

    // first is oldest, last is newest
    msgs.reverse();

    use ichiyo_ai::Completion as _;
    let result = match modelname {
        Modelname::GPT35Turbo => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiGPT35Turbo::new(token);

            tokio::time::timeout(TIMEOUT_DURATION, engine.next(&msgs)).await
        }
        Modelname::GPT4Turbo => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiGPT4Turbo::new(token);

            tokio::time::timeout(TIMEOUT_DURATION, engine.next(&msgs)).await
        }
        Modelname::GeminiPro => {
            let token = &crate::envs().google_ai_api_key;
            let engine = ichiyo_ai::GoogleGeminiPro::new(token);

            tokio::time::timeout(TIMEOUT_DURATION, engine.next(&msgs)).await
        }
    };

    let ret = match result {
        Ok(result) => (msg.link(), result?),
        Err(e) => {
            tracing::error!(?e, "Timeouted to generate completion");
            return Err("Timeouted to generate completion".into());
        }
    };

    let content = {
        let (link, (msg, meta)) = ret;

        let content = msg.content();
        let ichiyo_ai::CMetadata {
            tokens,
            price_yen,
            by,
        } = meta;

        format!("{content}\n\n**`{by}`** | ¥{price_yen:.2} | {tokens} tokens | from {link}")
    };

    let cirf = CreateInteractionResponseFollowup::default().content(content);
    ci.create_followup(ctx, cirf).await.map_err(|cause| {
        tracing::error!(?cause, "Failed to create interaction response");
        "Failed to create interaction response"
    })?;

    Ok(())
}

use cm::ChainedMessages;
mod cm {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::ready;
    use core::task::Context;
    use core::task::Poll;
    use serenity::model::channel::Message;
    use tokio_stream::Stream;

    pub struct ChainedMessages {
        ctx: serenity::client::Context,
        state: State,
    }

    enum State {
        Got { cursor: (u64, u64) },
        Pending { future: Pin<Box<GetMessage>> },
        Terminal,
    }

    impl State {
        fn kind(&self) -> &'static str {
            match self {
                Self::Got { .. } => "Got",
                Self::Pending { .. } => "Pending",
                Self::Terminal => "Terminal",
            }
        }
    }

    type GetMessage = dyn Future<Output = Option<Message>> + Send + Sync;

    impl ChainedMessages {
        pub fn new(ctx: serenity::client::Context, msg: &Message) -> Self {
            let channel_id = msg.channel_id.get();
            let message_id = msg.id.get();

            let cursor = (channel_id, message_id);
            let state = State::Got { cursor };

            Self { ctx, state }
        }
    }

    impl Stream for ChainedMessages {
        type Item = Message;

        #[tracing::instrument(skip_all)]
        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            tracing::debug!(state = ?self.state.kind(), "Polling on `ChainedMessages`");

            let (new, ret) = match &mut self.state {
                State::Got { cursor } => {
                    let (channel_id, message_id) = *cursor;

                    let ctx = self.ctx.clone();
                    let mut future = Box::pin(async move {
                        ctx.http
                            .get_message(channel_id.into(), message_id.into())
                            .await
                            .ok()
                    });

                    // FIXME: ↓ is this truthy true?

                    // initial polling. don't worry that `.poll(cx)` returns `Poll::Ready(_)`
                    // because if `Future` is already ready, `Future::poll(...)` will always return
                    // `Poll::Ready(_)`. it means when `Self::poll_next(...)` is called in future,
                    // then process it.
                    let polled = future.as_mut().poll(cx);
                    tracing::debug!(ret = ?polled, "Polled on `GetMessage`");

                    (State::Pending { future }, Poll::Pending)
                }
                State::Pending { future } => 'block: {
                    let Some(msg) = ready!(future.as_mut().poll(cx)) else {
                        break 'block (State::Terminal, Poll::Ready(None));
                    };

                    let option = msg
                        .referenced_message
                        .as_ref()
                        .map(|m| {
                            let channel_id = m.channel_id.get();
                            let message_id = m.id.get();

                            (channel_id, message_id)
                        })
                        .or_else(|| extract_reference_from_url(&msg.content));

                    let state = match option {
                        None => State::Terminal,
                        Some(cursor) => State::Got { cursor },
                    };

                    (state, Poll::Ready(Some(msg)))
                }
                State::Terminal => (State::Terminal, Poll::Ready(None)),
            };

            self.state = new;
            ret
        }
    }

    // HACK: 取り急ぎ実装したが, best practice かは知らない
    #[tracing::instrument(skip_all)]
    fn extract_reference_from_url(content: impl AsRef<str>) -> Option<(u64, u64)> {
        let content = content.as_ref();

        tracing::debug!(url = ?content, "Extracting reference from url");

        let (_, ids) = content.rsplit_once("https://discord.com/channels/@me/")?;
        let (channel_id, message_id) = ids.split_once('/')?;

        let channel_id = channel_id.parse().ok()?;
        let message_id = message_id.parse().ok()?;

        let reference = (channel_id, message_id);

        tracing::debug!(?reference, "Extracted reference");

        Some(reference)
    }
}

#[serenity::async_trait]
impl EventHandler for EvHandler {
    #[tracing::instrument(skip_all)]
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Some(ci) = interaction.as_command() else {
            return tracing::error!(?interaction, "Unexpected interaction");
        };

        if ci.user.bot || ci.user.system {
            let cirm = CreateInteractionResponseMessage::default().content("Only users can use");
            let cir = CreateInteractionResponse::Message(cirm);
            let result = ci.create_response(&ctx, cir).await;

            if let Err(cause) = result {
                return tracing::error!(?cause, "Failed to create interaction response");
            }
        }

        if let Err(cause) = ci.defer(&ctx).await {
            return tracing::error!(?cause, "Failed to defer interaction");
        }

        let result = match ci.data.kind {
            CommandType::ChatInput => image(&ctx, ci).await,
            CommandType::Message => completion(&ctx, ci).await,

            kind => {
                tracing::error!(kind = ?kind, "Unexpected command type");

                let cirm = CreateInteractionResponseMessage::default()
                    .content("Failed to recognize interaction");

                let cir = CreateInteractionResponse::Message(cirm);
                let result = ci.create_response(&ctx, cir).await;

                if let Err(cause) = result {
                    tracing::error!(?cause, "Failed to create interaction response");
                }

                return;
            }
        };

        if let Err(reason) = result {
            let ce = CreateEmbed::default()
                .title("Error")
                .description(reason)
                .color(0xff0000);

            let cirf = CreateInteractionResponseFollowup::default().add_embed(ce);
            let result = ci.create_followup(&ctx, cirf).await;

            if let Err(cause) = result {
                tracing::error!(?cause, "Failed to create interaction response");
            }
        }
    }

    #[tracing::instrument(skip_all)]
    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("Starting...");

        let version = env!("CARGO_PKG_VERSION");
        ctx.set_activity(Some(ActivityData::playing(format!("v{version}"))));

        tracing::info!(?version, "Running ichiyoAI");
        tracing::info!(username = %ready.user.name, user_id = %ready.user.id, "Connected to Discord API!");

        let guild_id = crate::envs().guild_id;
        let result = serde_json::from_str::<serde_json::Value>(include_str!("commands.json"));

        let map = match result {
            Ok(val) => val,
            Err(cause) => return tracing::error!(?cause, "Failed to parse `commands.json`"),
        };

        let result = ctx.http.create_guild_commands(guild_id.into(), &map).await;

        // TODO: will report this?
        let _ = match result {
            Ok(vec) => vec,
            Err(cause) => return tracing::error!(?cause, "Failed to create guild commands"),
        };

        tracing::info!(details = "none", "Created guild commands");
    }
}
