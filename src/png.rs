use std::io::{Cursor, Write};

use cairo::{Context, Format, ImageSurface, Rectangle};
use gio::{Cancellable, File, ReadInputStream};
use rsvg::{CairoRenderer, Loader};
use svg::Document;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("An IO error occurred: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to load SVG: {0}")]
    Loading(#[from] rsvg::LoadingError),
    #[error("Failed to render SVG: {0}")]
    Render(#[from] rsvg::RenderingError),
    #[error("Failed to create cairo surface: {0}")]
    Cairo(#[from] cairo::Error),
    #[error("Failed to output image data: {0}")]
    Output(#[from] cairo::IoError),
}

pub fn svg_to_png(
    svg_doc: Document,
    width: f64,
    height: f64,
    out: &mut impl Write,
) -> Result<(), Error> {
    let mut svg_buf = Vec::new();
    svg::write(&mut svg_buf, &svg_doc)?;
    let stream = ReadInputStream::new(Cursor::new(svg_buf));
    let svg_handle = Loader::new().read_stream(&stream, None::<&File>, None::<&Cancellable>)?;
    let renderer = CairoRenderer::new(&svg_handle);
    let surface = ImageSurface::create(Format::ARgb32, width as i32, height as i32)?;
    let context = Context::new(&*surface)?;
    let bounds = Rectangle::new(0.0, 0.0, width, height);
    renderer.render_document(&context, &bounds)?;
    surface.write_to_png(out)?;
    Ok(())
}
