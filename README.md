# FormationBot

FormationBot renders square dance formations as pictures, to aid discussions of calling. It is open source, under the MIT license.

This version of FormationBot is based off of [Alex Dehnert's original](https://gitlab.com/tech-squares/formationbot)

## Formatting language

FormationBot supports a mini-language to indicate formations. A person is represented by a single character, with optional prefixes to modify rendering.

Facing directions are indicated with:
- `<`, `>`, `^`, and `v` (or `V`) represent a person facing in the appropriate direction
- `nsew` can be used similarly (north, south, etc.) when more convenient (for example, mobile keyboards)
- `,`, `@`, and `*` represent a person with no facing direction
- `.` represents an empty position

Other common prefixes include:
- colors: `r`, `g`, `b`, `c`, `m`, and `y` for red, green, blue, cyan, magenta and yellow, respectively
- shape: `o` or `O` (lowercase or capital letter 'o') will make the dancer a circle
- outline styles: `-` or `:` will give the dancer a dashed or dotted outline, respectively (for backwards compatibility, `p` is supported as a synonym for `-`)
- labels: `x`, `X`, a digit, or `'` (apostrophe) followed by any character will fill the box in appropriately

For complicated formations, one trick is that each line is centered. If that's not enough, you can also prefix `u`, `d`, `l`, or `R` (yes, case-sensitive) to shift a person up or down a half matrix spot.

## Standalone usage

If you don't have it installed, [install Rust](https://www.rust-lang.org/tools/install).

Then, run `cargo run`. This program reads from stdin until end-of-file, and
then renders the formation in SVG to the file `out.svg`.

## Discord bot

Follow the instructions in [discord-bot/README.md](discord-bot/README.md)
