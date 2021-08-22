use std::mem;

use derivative::Derivative;

use crate::dancer::{Color, Dancer, Facing, Shape, StrokeStyle};

#[derive(Derivative, Debug, Clone, PartialEq)]
#[derivative(Default)]
pub struct Formation {
    pub dancers: Vec<Dancer>,
    #[derivative(Default(value="f64::NAN"))]
    pub min_x: f64,
    #[derivative(Default(value="f64::NAN"))]
    pub max_x: f64,
    #[derivative(Default(value="f64::NAN"))]
    pub min_y: f64,
    #[derivative(Default(value="f64::NAN"))]
    pub max_y: f64,
}

impl Formation {
    pub fn new(input: impl IntoIterator<Item = char>) -> Self {
        let mut formation = Formation::default();
        let mut line: Vec<Option<Dancer>> = Vec::new();
        let mut line_num = 0;
        let mut current = Dancer::default();
        let mut iter = input.into_iter();
        while let Some(c) = iter.next() {
            match c {
                // newline
                '/' | '\n' => {
                    formation.push_line(line_num, &mut line);
                    current = Dancer::default();
                    line_num += 1;
                }
                // colors
                'r' => current.color = Color::Red,
                'g' => current.color = Color::Green,
                'b' => current.color = Color::Blue,
                'c' => current.color = Color::Cyan,
                'm' => current.color = Color::Magenta,
                'y' => current.color = Color::Yellow,
                // adjustments
                'u' => current.y = -1.0,
                'd' => current.y = 1.0,
                'l' => current.x = -1.0,
                'R' => current.x = 1.0,
                // style
                'p' | '-' => current.stroke_style = StrokeStyle::Dashed,
                ':' => current.stroke_style = StrokeStyle::Dotted,
                // shape
                'o' | 'O' => current.shape = Shape::Circle,
                // label
                '0'..='9' | 'x' | 'X' => current.text = Some(c.into()),
                '\'' => {
                    if let Some(c) = iter.next() {
                        current.text = Some(c.into());
                    }
                }
                // facing direction (completes a dancer)
                'n' | '^' => {
                    current.facing = Facing::North;
                    line.push(Some(mem::take(&mut current)));
                }
                'e' | '>' => {
                    current.facing = Facing::East;
                    line.push(Some(mem::take(&mut current)));
                }
                's' | 'v' | 'V' => {
                    current.facing = Facing::South;
                    line.push(Some(mem::take(&mut current)));
                }
                'w' | '<' => {
                    current.facing = Facing::West;
                    line.push(Some(mem::take(&mut current)));
                }
                '+' => {
                    current.text = None;
                    current.shape = Shape::Plus;
                    line.push(Some(mem::take(&mut current)));
                }
                ',' | '@' | '*' => line.push(Some(mem::take(&mut current))),
                // empty spot (and clear current dancer attributes)
                '.' => {
                    current = Dancer::default();
                    line.push(None);
                }
                // unrecognized char; do nothing
                _ => {}
            }
        }
        formation.push_line(line_num, &mut line);
        formation
    }

    /// Returns the view box of the image: `(x, y, width, height)`
    pub fn view_box(&self) -> (f64, f64, f64, f64) {
        let width = self.max_x - self.min_x + 2.0;
        let height = self.max_y - self.min_y + 2.0;
        (self.min_x - 1.0, self.min_y - 1.0, width, height)
    }

    pub fn rendered_dimensions(&self) -> (f64, f64) {
        let (_, _, width, height) = self.view_box();
        (width * crate::render::DANCER_WIDTH, height * crate::render::DANCER_WIDTH)
    }

    fn push_line(&mut self, line_num: usize, line: &mut Vec<Option<Dancer>>) {
        let y0 = 2.0 * line_num as f64;
        // x position of first dancer
        let x0 = -(line.len() as f64);
        self.dancers.reserve(line.len());
        for (idx, dancer) in line.drain(..).enumerate() {
            let y;
            let x;
            if let Some(mut dancer) = dancer {
                dancer.y += y0;
                dancer.x += 2.0 * idx as f64 + x0;
                y = dancer.y;
                x = dancer.x;
                self.dancers.push(dancer);
            } else {
                y = y0;
                x = 2.0 * idx as f64 + x0;
            }
            self.min_x = self.min_x.min(x);
            self.max_x = self.max_x.max(x);
            self.min_y = self.min_y.min(y);
            self.max_y = self.max_y.max(y);
        }
    }
}
