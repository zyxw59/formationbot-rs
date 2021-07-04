use std::{borrow::Cow, iter::Peekable};

use formationbot::{parse::Formation, render::Render};
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::AttachmentType,
    model::channel::Message,
};

fn main() {
    println!("Hello, world!");
}

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
    async fn message(&self, ctx: Context, msg: Message) {
        let images = self
            .get_formations(&msg.content)
            .map(render_formation)
            .filter_map(|res| res.unwrap_or_default())
            .map(|png| AttachmentType::Bytes {
                data: Cow::Owned(png),
                filename: "formation.png".into(),
            })
            .peekable();
        send_reply(ctx, &msg, images).await.unwrap();
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
