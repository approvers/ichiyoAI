use anyhow::Context as _;
use once_cell::sync::OnceCell;
use serenity::builder::CreateAttachment;
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
    model::channel::Message,
    model::gateway::Ready,
};
use tracing::info;

use std::time::Duration;

// DaLL-E & ChatGPT で使用するタイムアウト時間の定数
pub static TIMEOUT_DURATION: Duration = Duration::from_secs(180);

pub async fn is_sponsor(ctx: &Context, user: &User) -> anyhow::Result<bool> {
    let envs = crate::envs();

    let guild_id = envs.guild_id;
    let role_id = envs.sponsor_role_id;

    user.has_role(ctx, guild_id, role_id)
        .await
        .context("Failed to get sponsor role")
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

async fn image(ctx: &Context, ci: &CommandInteraction) {
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

        Some(name) => unreachable!("Unexpected model name: {:?}", name),
        None => unreachable!("Model name is required"),
    };

    match (is_sponsor(ctx, &ci.user).await.unwrap(), &modelname) {
        (false, Modelname::DallE3) => panic!(),
        _ => (/* means for all users */),
    }

    let prompt = match opts.prompt {
        Some("") => unreachable!("empty prompt is not allowed"),
        Some(prompt) => prompt,
        None => unreachable!("prompt is required"),
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
        let (img, meta) = result.unwrap();

        let ichiyo_ai::GeneratedImage { image, prompt, ext } = img;
        let ichiyo_ai::dalle::Metadata { model } = meta;

        let filename = format!("image.{}", ext.as_str());
        let filedata = image;

        let content = format!("{model} - **`{prompt}`**");

        (content, [(filename, filedata)])
    };

    let cirf = CreateInteractionResponseFollowup::default()
        .content(content)
        .files(files.map(|(name, raw)| CreateAttachment::bytes(raw, name)));

    ci.create_followup(ctx, cirf).await.unwrap();
}

async fn completion(ctx: &Context, ci: &CommandInteraction) {
    enum Modelname {
        GPT35Turbo,
        GPT4Turbo,
        GeminiPro,
    }

    let modelname = match &*ci.data.name {
        "Text (GPT-3.5 Turbo)" => Modelname::GPT35Turbo,
        "Text (GPT-4 Turbo)" => Modelname::GPT4Turbo,
        "Text (Gemini Pro)" => Modelname::GeminiPro,

        name => unreachable!("Unexpected model name: {:?}", name),
    };

    match (is_sponsor(ctx, &ci.user).await.unwrap(), &modelname) {
        (false, Modelname::GPT4Turbo) => panic!(),
        _ => (/* means for all users */),
    }

    let Some(ResolvedTarget::Message(msg)) = ci.data.target() else {
        unreachable!("Unexpected target: {:?}", ci.data.target());
    };

    // first is newest, last is oldest
    use tokio_stream::StreamExt as _;
    let mut msgs = ChainedMessages::new(ctx.clone(), msg)
        .map(|m| {

            if m.is_own(ctx) {
                let (content, _) = m.content.rsplit_once("\n\n").unwrap();
                let content = content.to_owned();

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
        let (msg, meta) = result.unwrap();

        let content = msg.content();
        let ichiyo_ai::Metadata {
            tokens,
            price_yen,
            by,
        } = meta;

        format!("{content}\n\n`{by}` | ¥{price_yen:.2} | {tokens} tokens")
    };

    assert!(content.len() < 2000);

    let cirf = CreateInteractionResponseFollowup::default().content(content);
    ci.create_followup(ctx, cirf).await.unwrap();
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

    type GetMessage = dyn Future<Output = serenity::Result<Message>> + Send + Sync;

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
            dbg!({
                match &self.state {
                    State::Got { .. } => "Got",
                    State::Pending { .. } => "Pending",
                    State::Terminal => "Terminal",
                }
            });

            let (new, ret) = match &mut self.state {
                State::Got { cursor } => {
                    let (channel_id, message_id) = *cursor;

                    let ctx = self.ctx.clone();
                    let mut future = Box::pin(async move {
                        ctx.http
                            .get_message(channel_id.into(), message_id.into())
                            .await
                    });

                    let _ = future.as_mut().poll(cx);

                    (State::Pending { future }, Poll::Pending)
                }
                State::Pending { future } => {
                    let msg = ready!(future.as_mut().poll(cx));
                    if let Err(e) = &msg {
                        println!("{e}");
                    }

                    let msg = msg.unwrap();

                    let state = match msg.referenced_message.as_ref() {
                        None => State::Terminal,
                        Some(m) => {
                            let channel_id = m.channel_id.get();
                            let message_id = m.id.get();

                            let cursor = (channel_id, message_id);

                            State::Got { cursor }
                        }
                    };

                    (state, Poll::Ready(Some(msg)))
                }
                State::Terminal => (State::Terminal, Poll::Ready(None)),
            };

            self.state = new;
            ret
        }
    }
}

#[async_trait]
impl EventHandler for EvHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Some(ci) = interaction.as_command() else {
            unreachable!("Unexpected interaction: {:?}", interaction);
        };

        if ci.user.bot || ci.user.system {
            return;
        }

        ci.defer(&ctx).await.unwrap();

        match ci.data.kind {
            CommandType::ChatInput => image(&ctx, ci).await,
            CommandType::Message => completion(&ctx, ci).await,
            kind => unreachable!("Unexpected command type: {:?}", kind),
        }
    }

    async fn ready(&self, ctx: Context, self_bot: Ready) {
        info!("Starting...");

        let version = env!("CARGO_PKG_VERSION");
        ctx.set_activity(Some(ActivityData::playing(format!("v{version}"))));

        info!("Running ichiyoAI v{}", version);
        info!(
            "Connected!: {name}(Id:{id})",
            name = self_bot.user.name,
            id = self_bot.user.id
        );

        let guild_id = crate::envs().guild_id;
        let map = serde_json::from_str::<serde_json::Value>(include_str!("commands.json")).unwrap();

        let commands = ctx
            .http
            .create_guild_commands(guild_id.into(), &map)
            .await
            .unwrap();
    }
}
