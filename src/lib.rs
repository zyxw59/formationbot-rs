use svg::node::{
    element::{Circle, Definitions, Group, Rectangle, Text, Use},
    Node, Text as TextNode, Value,
};

/// In units of dancer width.
const NOSE_RADIUS: f64 = 3.0 / 16.0;
const STROKE_WIDTH: f64 = 1.0 / 16.0;
const DASH_LENGTH: f64 = 0.2;

pub fn definitions() -> Definitions {
    Definitions::new()
        .add(
            Circle::new()
                .set("id", "nose")
                .set("r", NOSE_RADIUS)
                .set("stroke", "none")
                .set("cx", 0)
                .set("cy", -0.5 - NOSE_RADIUS),
        )
        .add(
            Rectangle::new()
                .set("id", "dancer-square")
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", "none")
                .set("width", 1)
                .set("height", 1)
                .set("x", -0.5)
                .set("y", -0.5),
        )
        .add(
            Circle::new()
                .set("id", "dancer-circle")
                .set("stroke-width", STROKE_WIDTH)
                .set("fill", "none")
                .set("r", std::f64::consts::FRAC_1_PI.sqrt())
                .set("cx", 0)
                .set("cy", 0),
        )
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Dancer {
    pub x: f64,
    pub y: f64,
    pub color: Color,
    pub shape: Shape,
    pub stroke_style: StrokeStyle,
    pub facing: Facing,
    pub text: Option<String>,
}

impl Dancer {
    pub fn render(&self) -> Group {
        let mut group = Group::new().add(
            Use::new()
                .set("href", self.shape.href())
                .set("x", self.x)
                .set("y", self.y)
                .set("stroke", self.color)
                .set("stroke-dasharray", self.stroke_style),
        );
        match self.facing {
            Facing::None => {}
            _ => group.append(
                Use::new()
                    .set("href", "#nose")
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
            group.append(
                Text::new()
                    .set("fill", self.color)
                    .set("x", self.x)
                    .set("y", self.y)
                    .set("font-size", 1)
                    .set("text-anchor", "middle")
                    .set("dominant-baseline", "central")
                    .add(TextNode::new(text)),
            )
        }
        group
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Rgb(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Color {
        Color::Black
    }
}

impl From<Color> for Value {
    fn from(color: Color) -> Value {
        format!("{:?}", color).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
    Square,
    Circle,
}

impl Default for Shape {
    fn default() -> Shape {
        Shape::Square
    }
}

impl Shape {
    fn href(&self) -> &'static str {
        match self {
            Shape::Square => "#dancer-square",
            Shape::Circle => "#dancer-circle",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum StrokeStyle {
    Solid,
    Dotted,
    Dashed,
}

impl Default for StrokeStyle {
    fn default() -> StrokeStyle {
        StrokeStyle::Solid
    }
}

impl From<StrokeStyle> for Value {
    fn from(stroke_style: StrokeStyle) -> Value {
        use self::StrokeStyle::*;
        match stroke_style {
            Solid => "none".into(),
            Dotted => STROKE_WIDTH.into(),
            Dashed => DASH_LENGTH.into(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Facing {
    None,
    North,
    East,
    South,
    West,
    Angle(f64),
}

impl Facing {
    fn angle(&self) -> f64 {
        use self::Facing::*;
        match self {
            None | North => 0.0,
            East => 90.0,
            South => 180.0,
            West => 270.0,
            Angle(x) => *x,
        }
    }
}

impl Default for Facing {
    fn default() -> Facing {
        Facing::North
    }
}

#[cfg(test)]
mod tests {}
