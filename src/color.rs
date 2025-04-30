pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub const fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }
}

impl From<Color> for inquire::ui::Color {
    fn from(value: Color) -> Self {
        inquire::ui::Color::Rgb {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

impl From<Color> for colored::CustomColor {
    fn from(value: Color) -> Self {
        colored::CustomColor {
            r: value.r,
            g: value.g,
            b: value.b,
        }
    }
}

pub const RUST_ORANGE: Color = Color::new(247, 76, 0);

pub const JET_BLACK: Color = Color::new(0, 0, 0);

pub const PURE_WHITE: Color = Color::new(255, 255, 255);

pub const STEEL_GRAY: Color = Color::new(51, 51, 51);

pub const FERRIS_TEAL: Color = Color::new(0, 163, 163);

pub const EMBER_RED: Color = Color::new(183, 65, 14);
