use svg::{
    node::{
        element::{path::Data, Circle, Definitions, Group, Path, Rectangle, Text, Use},
        Node, Text as TextNode,
    },
    Document,
};

use crate::{dancer, parse::Formation};

/// In units of dancer width.
pub const NOSE_RADIUS: f64 = 3.0 / 16.0;
pub const DANCER_RADIUS: f64 = std::f64::consts::FRAC_2_SQRT_PI / 2.0;
pub const STROKE_WIDTH: f64 = 1.0 / 16.0;
pub const DASH_LENGTH: f64 = 0.2;
pub const DANCER_SQUARE_REF: &str = "#dancer-square";
pub const DANCER_SQUARE_ID: &str = "dancer-square";
pub const DANCER_CIRCLE_REF: &str = "#dancer-circle";
pub const DANCER_CIRCLE_ID: &str = "dancer-circle";
pub const DANCER_PLUS_REF: &str = "#dancer-plus";
pub const DANCER_PLUS_ID: &str = "dancer-plus";
pub const NOSE_REF: &str = "#nose";
pub const NOSE_ID: &str = "nose";

/// Dancer width, in pixels.
pub const DANCER_WIDTH: f64 = 100.0;

pub fn definitions() -> Definitions {
    Definitions::new()
        .add(
            Circle::new()
                .set("id", NOSE_ID)
                .set("r", NOSE_RADIUS)
                .set("stroke", "none")
                .set("cx", 0)
                .set("cy", -0.5 - NOSE_RADIUS),
        )
        .add(
            Rectangle::new()
                .set("id", DANCER_SQUARE_ID)
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", "none")
                .set("width", 1)
                .set("height", 1)
                .set("x", -0.5)
                .set("y", -0.5),
        )
        .add(
            Circle::new()
                .set("id", DANCER_CIRCLE_ID)
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", "none")
                .set("r", DANCER_RADIUS)
                .set("cx", 0)
                .set("cy", 0),
        )
        .add(
            Path::new()
                .set("id", DANCER_PLUS_ID)
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", "none")
                .set(
                    "d",
                    Data::new()
                        .move_to((-DANCER_RADIUS, 0.0))
                        .horizontal_line_by(DANCER_RADIUS * 2.0)
                        .move_to((0.0, -DANCER_RADIUS))
                        .vertical_line_by(DANCER_RADIUS * 2.0),
                ),
        )
}

pub trait Render {
    type Output;

    fn render(&self) -> Self::Output;
}

impl Render for dancer::Dancer {
    type Output = Group;

    fn render(&self) -> Self::Output {
        let mut group = Group::new().add(
            Use::new()
                .set("href", self.shape.href())
                .set("x", self.x)
                .set("y", self.y)
                .set("stroke", self.color)
                .set("stroke-dasharray", self.stroke_style),
        );
        match self.facing {
            dancer::Facing::None => {}
            _ => group.append(
                Use::new()
                    .set("href", NOSE_REF)
                    .set("x", self.x)
                    .set("y", self.y)
                    .set("fill", self.color)
                    .set(
                        "transform",
                        format!("rotate({} {} {})", self.facing.angle(), self.x, self.y),
                    ),
            ),
        };
        if let Some(text) = &self.text {
            // workaround for rsvg not supporting `dominant-baseline`
            let (y, baseline) = if cfg!(feature = "png") {
                (self.y + 0.35, "auto")
            } else {
                (self.y, "central")
            };
            group.append(
                Text::new()
                    .set("fill", self.color)
                    .set("x", self.x)
                    .set("y", y)
                    .set("font-size", 1)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", baseline)
                    .add(TextNode::new(text)),
            )
        }
        group
    }
}

impl Render for Formation {
    type Output = Document;

    fn render(&self) -> Self::Output {
        let (width, height) = self.rendered_dimensions();
        let mut doc = Document::new()
            .set("viewBox", self.view_box())
            .set("height", height)
            .set("width", width)
            .add(definitions());
        for dancer in &self.dancers {
            doc.append(dancer.render())
        }
        doc
    }
}
