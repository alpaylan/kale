use std::collections::HashMap;

use macroquad::{
    shapes::draw_line,
    text::{measure_text, Font, TextDimensions},
};
use pest::Position;

use crate::{
    dom::{DOMAction, DOMElement, DOM},
    styling::{Display, FontFamily, FontWeight, TextDecorationLine},
};

pub fn render_dom(
    dom: &DOM,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color, &Font) -> TextDimensions,
    fonts: &HashMap<(FontFamily, FontWeight), Font>,
) -> Vec<(BoundingBox, Vec<DOMAction>, String)> {
    macroquad::window::clear_background(macroquad::color::WHITE);

    let bbox = BoundingBox {
        x: 0.0,
        y: 0.0,
        width: macroquad::window::screen_width(),
        height: macroquad::window::screen_height(),
    };

    let mut position = Point { x: 0.0, y: 0.0 };

    let mut element_boxes = vec![];

    for element in dom.elements.iter() {
        position = render_dom_element(
            element,
            bbox,
            position,
            draw_text,
            fonts,
            &mut element_boxes,
        );
    }

    element_boxes
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl BoundingBox {
    pub(crate) fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x <= self.x + self.width
            && point.y >= self.y
            && point.y <= self.y + self.height
    }
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

impl From<(f32, f32)> for Point {
    fn from((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

pub(crate) fn render_dom_element(
    element: &DOMElement,
    bbox: BoundingBox,
    position: Point,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color, &Font) -> TextDimensions,
    fonts: &HashMap<(FontFamily, FontWeight), Font>,
    element_boxes: &mut Vec<(BoundingBox, Vec<DOMAction>, String)>,
) -> Point {
    let mut cursor = position;
    let mut bbox = bbox;
    match element {
        DOMElement::View {
            style,
            children,
            actions,
            id,
            ..
        } => match style.display {
            Display::Block => {
                let line_height = style.font.size.to_pixels(16.0);
                let margin_top = style.margin.top.to_pixels(line_height);
                let margin_left = style.margin.left.to_pixels(line_height);

                cursor.y += line_height + margin_top;
                bbox.x += margin_left;
                cursor.x = bbox.x;
                for child in children {
                    cursor =
                        render_dom_element(child, bbox, cursor, draw_text, fonts, element_boxes);
                }
                let margin_bottom = style.margin.bottom.to_pixels(line_height);
                let margin_right = style.margin.right.to_pixels(line_height);

                element_boxes.push((
                    BoundingBox {
                        x: position.x,
                        y: position.y,
                        width: cursor.x - position.x + margin_right,
                        height: cursor.y - position.y + margin_bottom,
                    },
                    actions.clone(),
                    id.clone(),
                ));

                Point::new(position.x + margin_right, cursor.y + margin_bottom)
            }
            Display::Inline => {
                let mut cursor = position;
                for child in children {
                    cursor =
                        render_dom_element(child, bbox, cursor, draw_text, fonts, element_boxes);
                }

                element_boxes.push((
                    BoundingBox {
                        x: position.x,
                        y: position.y,
                        width: cursor.x - position.x,
                        height: cursor.y - position.y,
                    },
                    actions.clone(),
                    id.clone(),
                ));

                Point::new(cursor.x, cursor.y)
            }
        },
        DOMElement::Text {
            text,
            style,
            actions,
            id
        } => {
            // Tokenization
            let tokens = text.split_whitespace();
            let line_height = style.font.size.to_pixels(16.0);
            let space_width = measure_text(
                " ",
                fonts.get(&(style.font.family, style.font.weight)),
                style.font.size.to_pixels(16.0).round() as u16,
                1.0,
            );

            for token in tokens {
                let dimensions = measure_text(
                    token,
                    fonts.get(&(style.font.family, style.font.weight)),
                    line_height.round() as u16,
                    1.0,
                );

                if cursor.x + dimensions.width > bbox.width {
                    cursor.y += line_height;
                    cursor.x = bbox.x;
                }

                if let TextDecorationLine::Underline = style.text_decoration.line {
                    draw_line(
                        cursor.x,
                        cursor.y,
                        cursor.x + dimensions.width,
                        cursor.y,
                        1.0,
                        style.text_decoration.color.into(),
                    );
                }

                draw_text(
                    &token,
                    cursor.x,
                    cursor.y,
                    line_height.round() as u16,
                    style.color.into(),
                    fonts.get(&(style.font.family, style.font.weight)).unwrap(),
                );

                cursor.x += dimensions.width + space_width.width;
            }

            element_boxes.push((
                BoundingBox {
                    x: position.x,
                    y: position.y,
                    width: cursor.x - position.x,
                    height: cursor.y - position.y,
                },
                actions.clone(),
                id.clone(),
            ));

            Point::new(cursor.x, cursor.y)
        }
    }
}
