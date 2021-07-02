use formationbot::{definitions, Dancer, Facing, Shape, StrokeStyle, Color};
use svg::{node::Node, save, Document};

fn main() {
    let mut doc = Document::new()
        .set("viewBox", (-2, -2, 4, 4))
        .set("height", 400)
        .set("width", 400)
        .add(definitions());
    let dancers = vec![
        Dancer {
            x: -1.0,
            y: -1.0,
            facing: Facing::South,
            stroke_style: StrokeStyle::Dotted,
            text: Some("4".into()),
            ..Default::default()
        },
        Dancer {
            x: 1.0,
            y: -1.0,
            facing: Facing::None,
            text: Some("3".into()),
            ..Default::default()
        },
        Dancer {
            x: 1.0,
            y: 1.0,
            facing: Facing::North,
            stroke_style: StrokeStyle::Dashed,
            color: Color::Blue,
            text: Some("2".into()),
            ..Default::default()
        },
        Dancer {
            x: -1.0,
            y: 1.0,
            facing: Facing::East,
            shape: Shape::Circle,
            text: Some("1".into()),
            ..Default::default()
        },
    ];
    for dancer in dancers {
        doc.append(dancer.render())
    }

    save("test.svg", &doc).unwrap();
}
