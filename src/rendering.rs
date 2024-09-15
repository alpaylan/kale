use macroquad::{
    shapes::draw_line,
    text::{measure_text, Font, TextDimensions},
};
use pest::Position;

use crate::{
    dom::{DOMElement, DOM},
    styling::{Display, TextDecorationLine},
};

pub fn render_dom(
    dom: &DOM,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
    font: &Font,
) {
    macroquad::window::clear_background(macroquad::color::WHITE);

    let bbox = BoundingBox {
        x: 0.0,
        y: 0.0,
        width: macroquad::window::screen_width(),
        height: macroquad::window::screen_height(),
    };

    let mut position = Point { x: 0.0, y: 0.0 };

    for element in dom.elements.iter() {
        position = render_dom_element(element, bbox, position, draw_text, font);
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub(crate) fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub(crate) fn render_dom_element(
    element: &DOMElement,
    bbox: BoundingBox,
    position: Point,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
    font: &Font,
) -> Point {
    let mut cursor = position;

    match element {
        DOMElement::View {
            style, children, ..
        } => match style.display {
            Display::Block => {
                let line_height = style.font.size.to_pixels(16.0);
                cursor.y += line_height;
                cursor.x = bbox.x;
                for child in children {
                    cursor = render_dom_element(child, bbox, cursor, draw_text, font);
                }
                
                Point::new(position.x, cursor.y + line_height)
            }
            Display::Inline => {
                let mut cursor = position;
                for child in children {
                    cursor = render_dom_element(child, bbox, cursor, draw_text, font);
                }

                Point::new(cursor.x, cursor.y)
            }
        },
        DOMElement::Text { text, style, .. } => {
            // Tokenization
            let tokens = text.split_whitespace();
            let line_height = style.font.size.to_pixels(16.0);
            let space_width = measure_text(
                " ",
                Some(font),
                style.font.size.to_pixels(16.0).round() as u16,
                1.0,
            );

            for token in tokens {
                let dimensions = measure_text(token, Some(font), line_height.round() as u16, 1.0);

                if cursor.x + dimensions.width > bbox.width {
                    cursor.y += line_height;
                    cursor.x = bbox.x;
                }

                draw_text(
                    &token,
                    cursor.x,
                    cursor.y + line_height,
                    line_height.round() as u16,
                    style.color.into(),
                );
                
                if let TextDecorationLine::Underline = style.text_decoration.line {
                    draw_line(
                        cursor.x,
                        cursor.y + line_height,
                        cursor.x + dimensions.width,
                        cursor.y + line_height,
                        1.0,
                        style.text_decoration.color.into(),
                    );
                }

                cursor.x += dimensions.width + space_width.width;
            }

            Point::new(cursor.x, cursor.y)
        }
    }
}
