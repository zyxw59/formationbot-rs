use std::{borrow::Cow, iter::Peekable};

use formationbot::{parse::Formation, render::Render};
use log::LevelFilter;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    http::AttachmentType,
    model::{channel::Message, gateway::Ready},
};

#[derive(serde::Deserialize)]
struct Config {
    discord_token: String,
    #[serde(flatten)]
    handler: Handler,
    log_level: LevelFilter,
}

impl Config {
    fn new() -> anyhow::Result<Self> {
        let file = std::fs::File::open("./config.yaml")?;
        serde_yaml::from_reader::<_, Self>(file).map_err(Into::into)
    }

    fn init_logging(&self) {
        let mut builder = pretty_env_logger::formatted_builder();
        builder.filter_level(self.log_level);
        if self.log_level < LevelFilter::Trace {
            builder.filter(Some("tracing"), LevelFilter::Off);
            builder.filter(Some("serenity"), LevelFilter::Off);
        }
        builder.init();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::new()?;
    config.init_logging();
    let mut client = Client::builder(config.discord_token)
        .event_handler(config.handler)
        .await?;
    client.start().await?;
    Ok(())
}

#[derive(Clone, Debug, serde::Deserialize)]
struct Handler {
    start_tag: String,
    end_tag: String,
    comment_tag: String,
}

impl Handler {
    fn get_formations<'tags, 'msg>(&'tags self, mut msg: &'msg str) -> MessageIter<'tags, 'msg> {
        if let Some((text, _)) = msg.split_once(&self.comment_tag) {
            msg = text;
        }
        MessageIter {
            start_tag: &self.start_tag,
            end_tag: &self.end_tag,
            msg,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        log::info!("Client ready in {} guilds", ready.guilds.len());
    }

    async fn message(&self, ctx: Context, msg: Message) {
        log::info!("received message: {}", msg.content);
        let images = self
            .get_formations(&msg.content)
            .map(render_formation)
            .filter_map(|res| {
                match res {
                    Ok(ok) => ok,
                    Err(e) => {
                        log::error!("Failed to render formation: {}", e);
                        None
                    }
                }
            })
            .enumerate()
            .map(|(idx, png)| AttachmentType::Bytes {
                data: Cow::Owned(png),
                filename: format!("formation-{}.png", idx),
            })
            .peekable();
        if let Err(e) = send_reply(ctx, &msg, images).await {
            log::error!("Failed to send reply: {}", e);
        }
    }
}

/// Renders the formation text to an image in PNG format.
fn render_formation(formation_text: &str) -> anyhow::Result<Option<Vec<u8>>> {
    let formation = Formation::new(formation_text.chars());
    if formation.dancers.is_empty() {
        Ok(None)
    } else {
        let (width, height) = formation.rendered_dimensions();
        let bounds = cairo::Rectangle {
            x: 0.0,
            y: 0.0,
            width,
            height,
        };
        let svg_doc = formation.render();
        let mut buf = Vec::new();
        svg::write(&mut buf, &svg_doc)?;
        let stream = gio::ReadInputStream::new(std::io::Cursor::new(buf));
        let svg_handle = librsvg::Loader::new().read_stream(
            &stream,
            None::<&gio::File>,
            None::<&gio::Cancellable>,
        )?;
        let renderer = librsvg::CairoRenderer::new(&svg_handle);
        let surface =
            cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32)?;
        let context = cairo::Context::new(&*surface)?;
        renderer.render_document(&context, &bounds)?;
        let mut out_buf = Vec::new();
        surface.write_to_png(&mut out_buf)?;
        Ok(Some(out_buf))
    }
}

async fn send_reply(
    ctx: Context,
    msg: &Message,
    mut attachments: Peekable<impl Iterator<Item = AttachmentType<'_>>>,
) -> anyhow::Result<()> {
    if attachments.peek().is_some() {
        msg.channel_id
            .send_message(&ctx.http, |m| {
                m.reference_message(msg);
                m.files(attachments);
                m
            })
            .await?;
    }
    Ok(())
}

struct MessageIter<'tags, 'msg> {
    start_tag: &'tags str,
    end_tag: &'tags str,
    msg: &'msg str,
}

impl<'tags, 'msg> Iterator for MessageIter<'tags, 'msg> {
    type Item = &'msg str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((_, formation)) = self.msg.split_once(self.start_tag) {
            if let Some((formation, rest)) = formation.split_once(self.end_tag) {
                self.msg = rest;
                Some(formation)
            } else {
                self.msg = "";
                Some(formation)
            }
        } else {
            self.msg = "";
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::{render_formation, Handler};

    #[test]
    fn message_iter() {
        let text = "first /f >>/<< f/ second /f >> /f << f/ third /f ^^ :// comment";
        let handler = Handler {
            start_tag: "/f".into(),
            end_tag: "f/".into(),
            comment_tag: "://".into(),
        };

        assert_eq!(
            handler.get_formations(text).collect::<Vec<_>>(),
            &[" >>/<< ", " >> /f << ", " ^^ "]
        );

        let text = "first /f>>/<<f/ second /f<>/<>";
        assert_eq!(
            handler.get_formations(text).collect::<Vec<_>>(),
            &[">>/<<", "<>/<>"]
        );
    }

    #[test]
    fn render() {
        render_formation("<>/><").unwrap().unwrap();
        assert!(render_formation("abcd").unwrap().is_none());
    }
}
