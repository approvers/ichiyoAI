use anyhow::Result;
use anyhow::bail;
use anyhow::Context as _;
use once_cell::sync::OnceCell;
use serenity::builder::CreateAttachment;
use serenity::builder::CreateInteractionResponse;
use serenity::builder::CreateInteractionResponseFollowup;
use serenity::builder::CreateInteractionResponseMessage;
use serenity::model::application::CommandDataOptionValue;
use serenity::model::application::CommandType;
use serenity::model::application::{CommandInteraction, Interaction, ResolvedTarget};
use serenity::model::user::User;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    gateway::ActivityData,
    model::gateway::Ready,
};
use tracing::info;

use std::time::Duration;

// DaLL-E & ChatGPT で使用するタイムアウト時間の定数
pub static TIMEOUT_DURATION: Duration = Duration::from_secs(180);

pub async fn is_sponsor(ctx: &Context, user: &User) -> Result<bool> {
    let envs = crate::envs();

    let guild_id = envs.guild_id;
    let role_id = envs.sponsor_role_id;

    Ok(user.has_role(ctx, guild_id, role_id).await?)
}

pub struct EvHandler;

pub static SYSTEM_CONTEXT: &str = "回答時は以下のルールに従うこと.\n- 1900文字以内に収めること。";
pub static OWN_MENTION: OnceCell<String> = OnceCell::new();

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

            _ => None,
        })
        .collect::<Options>();

    let modelname = match opts.model {
        Some("dall-e-2") => Modelname::DallE2,
        Some("dall-e-3") => Modelname::DallE3,

        name => bail!("Unexpected `model`: {}", name.unwrap_or("")),
    };

    let is_sponsor = is_sponsor(ctx, &ci.user)
        .await
        .context("Failed to get sponsor role")?;

    match (is_sponsor, &modelname) {
        (false, Modelname::DallE3) => bail!("You must be a sponsor to use model \"DALL-E 3\""),
        _ => (/* means for all users */),
    }

    let prompt = match opts.prompt {
        Some("") | None => bail!("Unexpected empty `prompt`"),
        Some(prompt) => prompt,
    };

    use ichiyo_ai::Image as _;
    let result = match modelname {
        Modelname::DallE2 => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiDallE2::new(token);

            engine.create(prompt).await
        }
        Modelname::DallE3 => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiDallE3::new(token);

            engine.create(prompt).await
        }
    };

    let (content, files) = {
        let (img, meta) = result.context("Failed to create image")?;

        let ichiyo_ai::GeneratedImage { image, prompt, ext } = img;
        let ichiyo_ai::dalle::Metadata { model } = meta;

        let filename = format!("image.{}", ext.as_str());
        let filedata = image;

        let content = format!("{model} - **`{prompt}`**");

        (content, [(filename, filedata)])
    };

    if content.len() > 2000 {
        bail!("Unexpected content length (too long): {}", content.len());
    }

    let cirf = CreateInteractionResponseFollowup::default()
        .content(content)
        .files(files.map(|(name, raw)| CreateAttachment::bytes(raw, name)));

    ci.create_followup(ctx, cirf)
        .await
        .context("Failed to create interaction response")?;

    Ok(())
}

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

        name => bail!("Unexpected command name: {name}"),
    };

    let is_sponsor = is_sponsor(ctx, &ci.user)
        .await
        .context("Failed to get sponsor role")?;

    match (is_sponsor, &modelname) {
        (false, Modelname::GPT4Turbo) => {
            bail!("You must be a sponsor to use model \"GPT-4 Turbo\"")
        }
        _ => (/* means for all users */),
    }

    let Some(ResolvedTarget::Message(msg)) = ci.data.target() else {
        bail!("Unexpected target: {:?}", ci.data.target());
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

            engine.next(&msgs).await
        }
        Modelname::GPT4Turbo => {
            let token = &crate::envs().openai_api_key;
            let engine = ichiyo_ai::OpenAiGPT4Turbo::new(token);

            engine.next(&msgs).await
        }
        Modelname::GeminiPro => {
            let token = &crate::envs().google_ai_api_key;
            let engine = ichiyo_ai::Gemini::new(token);

            engine.next(&msgs).await
        }
    };

    let content = {
        let link = msg.link();

        let (msg, meta) = result.context("Failed to complete")?;

        let content = msg.content();
        let ichiyo_ai::Metadata {
            tokens,
            price_yen,
            by,
        } = meta;

        format!("{content}\n\n**`{by}`** | ¥{price_yen:.2} | {tokens} tokens | from {link}")
    };

    // Internal Error
    if content.len() > 2000 {
        bail!("Unexpected content length (too long): {}", content.len());
    }

    let cirf = CreateInteractionResponseFollowup::default().content(content);
    let _ = ci
        .create_followup(ctx, cirf)
        .await
        .context("Failed to create interaction response")?;

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
    use tracing::trace;

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

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            trace!(
                "Polling on `ChainedMessages`: state = {}",
                self.state.kind()
            );

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
                    let _ = future.as_mut().poll(cx);

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
    fn extract_reference_from_url(content: impl AsRef<str>) -> Option<(u64, u64)> {
        let content = content.as_ref();

        let (_, ids) = content.rsplit_once("https://discord.com/channels/@me/")?;
        let (channel_id, message_id) = ids.split_once('/')?;

        let channel_id = channel_id.parse().ok()?;
        let message_id = message_id.parse().ok()?;

        Some((channel_id, message_id))
    }
}

use tracing::error;

#[async_trait]
impl EventHandler for EvHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        // Internal Error
        let Some(ci) = interaction.as_command() else {
            return error!("Unexpected interaction: {interaction:?}");
        };

        // Request Error
        if ci.user.bot || ci.user.system {
            let cirm = CreateInteractionResponseMessage::default().content("Only users can use");
            let cir = CreateInteractionResponse::Message(cirm);
            let result = ci.create_response(&ctx, cir).await;

            if let Err(e) = result {
                return error!("Failed to create interaction response: {e}");
            }
        }

        // Internal Error
        if let Err(e) = ci.defer(&ctx).await {
            return error!("Failed to create interaction response: {e}");
        }

        let result = match ci.data.kind {
            CommandType::ChatInput => image(&ctx, ci).await,
            CommandType::Message => completion(&ctx, ci).await,

            // Internal Error
            kind => {
                let content = format!("Unexpected command type: {kind:?}",);
                let cirm = CreateInteractionResponseMessage::default().content(&content);
                let cir = CreateInteractionResponse::Message(cirm);
                let result = ci.create_response(&ctx, cir).await;

                // Internal Error
                if let Err(e) = result {
                    error!("Failed to create interaction response: {e}");
                }

                return error!("{content}");
            }
        };

        // Internal Error
        if let Err(e) = result {
            let cirf = CreateInteractionResponseFollowup::default().content(format!("```{e}```"));
            let result = ci.create_followup(&ctx, cirf).await;

            // Internal Error
            if let Err(e) = result {
                error!("Failed to create interaction response: {e}");
            }

            error!("{e}");
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("Starting...");

        let version = env!("CARGO_PKG_VERSION");
        ctx.set_activity(Some(ActivityData::playing(format!("v{version}"))));

        info!("Running ichiyoAI v{version}");
        info!("Connected!: {}(Id:{})", ready.user.name, ready.user.id);

        let guild_id = crate::envs().guild_id;
        let map = match serde_json::from_str::<serde_json::Value>(include_str!("commands.json")) {
            Ok(val) => val,
            Err(e) => return error!("Failed to parse `commands.json`: {e}"),
        };

        // TODO: will report this?
        let _ = match ctx.http.create_guild_commands(guild_id.into(), &map).await {
            Ok(vec) => vec,
            Err(e) => return error!("Failed to create guild commands: {e}"),
        };

        info!("Created guild commands: <no details provided>");
    }
}
