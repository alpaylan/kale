
#[derive(Debug)]
pub(crate) struct Style {
    pub display: Display,
    pub margin: Margin,
    pub font: Font,
    pub color: Color,
}

impl Style {
    pub(crate) fn new(display: Display, margin: Margin, font: Font, color: Color) -> Self {
        Self {
            display,
            margin,
            font,
            color,
        }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self {
            display: Display::Block,
            margin: Margin::default(),
            font: Font::default(),
            color: Color::default(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Display {
    Block,
    Inline,
}

impl Default for Display {
    fn default() -> Self {
        Self::Block
    }
}

#[derive(Debug)]
pub(crate) struct Margin {
    pub top: Unit,
    pub right: Unit,
    pub bottom: Unit,
    pub left: Unit,
}

impl Margin {
    pub(crate) fn new(top: Unit, right: Unit, bottom: Unit, left: Unit) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}

impl Default for Margin {
    fn default() -> Self {
        Self {
            top: Unit::Px(0.0),
            right: Unit::Px(0.0),
            bottom: Unit::Px(0.0),
            left: Unit::Px(0.0),
        }
    }
}

#[derive(Debug)]
pub(crate) enum Unit {
    Px(f32),
    Em(f32),
    Rem(f32),
}

impl Unit {
    pub(crate) fn to_pixels(&self) -> f32 {
        match self {
            Self::Px(px) => *px,
            Self::Em(m) | Self::Rem(m) => 16.0 * m,
        }
    }
}

impl From<Unit> for f32 {
    fn from(unit: Unit) -> Self {
        unit.to_pixels()
    }
}

impl From<Unit> for u16 {
    fn from(unit: Unit) -> Self {
        unit.to_pixels().round() as u16
    }
}

#[derive(Debug)]
pub(crate) struct Font {
    pub size: Unit,
    pub family: String,
    pub weight: FontWeight,
    pub style: FontStyle,
}

impl Font {
    pub(crate) fn new(size: Unit, family: String, weight: FontWeight, style: FontStyle) -> Self {
        Self {
            size,
            family,
            weight,
            style,
        }
    }
}


impl Default for Font {
    fn default() -> Self {
        Self {
            size: Unit::Px(16.0),
            family: "Times New Roman".to_string(),
            weight: FontWeight::Normal,
            style: FontStyle::Normal,
        }
    }
}

#[derive(Debug)]
pub(crate) enum FontWeight {
    Normal,
    Bold,
}

impl Default for FontWeight {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug)]
pub(crate) enum FontStyle {
    Normal,
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Debug)]
pub(crate) struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }
}

impl From<Color> for macroquad::color::Color {
    fn from(color: Color) -> Self {
        macroquad::color::Color::from_rgba(color.r, color.g, color.b, color.a)
    }
}