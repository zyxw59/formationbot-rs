use std::io::Read;

use formationbot::{
    parse::Formation,
    render::Render,
};

fn main() -> std::io::Result<()> {
    let mut stdin = std::io::stdin();
    let mut buf = String::new();
    stdin.read_to_string(&mut buf)?;
    let formation = Formation::new(buf.chars());

    svg::save("out.svg", &formation.render())
}
