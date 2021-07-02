use svg::node::Value;

use crate::render;

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
    pub fn href(&self) -> &'static str {
        match self {
            Shape::Square => render::DANCER_SQUARE_REF,
            Shape::Circle => render::DANCER_CIRCLE_REF,
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
        match stroke_style {
            StrokeStyle::Solid => "none".into(),
            StrokeStyle::Dotted => render::STROKE_WIDTH.into(),
            StrokeStyle::Dashed => render::DASH_LENGTH.into(),
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
    pub fn angle(&self) -> f64 {
        match self {
            Facing::None | Facing::North => 0.0,
            Facing::East => 90.0,
            Facing::South => 180.0,
            Facing::West => 270.0,
            Facing::Angle(x) => *x,
        }
    }
}

impl Default for Facing {
    fn default() -> Facing {
        Facing::North
    }
}
