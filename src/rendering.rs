use macroquad::{
    color::Color,
    math::{quat, vec2},
    shapes::draw_line,
    text::{self, measure_text, Font, TextDimensions},
};

use crate::{
    dom::{DOMElement, DOM},
    styling::{Display, Style, TextDecoration, TextDecorationLine},
};

pub fn render_dom(
    dom: &DOM,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
    font: &Font,
) {
    macroquad::window::clear_background(macroquad::color::WHITE);

    let mut bbox = BoundingBox {
        x: 0.0,
        y: 0.0,
        width: macroquad::window::screen_width(),
        height: macroquad::window::screen_height(),
    };

    for element in dom.elements.iter() {
        let result_box = render_dom_element(element, bbox, draw_text, font);
        bbox.y += result_box.height;
        bbox.height -= result_box.height;
    }
}

#[derive(Debug, Copy, Clone)]
struct BoundingBox {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

pub(crate) fn render_dom_element(
    element: &DOMElement,
    bbox: BoundingBox,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
    font: &Font,
) -> BoundingBox {
    match element {
        DOMElement::View {
            style, children, ..
        } => match style.display {
            Display::Block => {
                let mut bbox = bbox;
                for child in children {
                    let result_box = render_dom_element(child, bbox, draw_text, font);
                    bbox.y += result_box.height;
                }

                bbox
            }
            Display::Inline => {
                let text_children: Vec<DOMElement> =
                    children.into_iter().map(|child| child.to_text()).collect();

                let mut x = 0.0;
                let line_height = style.font.size.to_pixels();
                let mut y = line_height;
                let space_width = measure_text(
                    " ",
                    Some(font),
                    style.font.size.to_pixels().round() as u16,
                    1.0,
                );

                for text_child in text_children {
                    // Tokenization
                    let text_child = text_child.unchecked_text();
                    let tokens = text_child.split_whitespace();

                    for token in tokens {
                        let dimensions =
                            measure_text(token, Some(font), line_height.round() as u16, 1.0);

                        if x + dimensions.width > bbox.width {
                            y += line_height;
                            x = 0.0;
                        }

                        draw_text(
                            &token,
                            bbox.x + x,
                            bbox.y + y,
                            line_height.round() as u16,
                            style.color.into(),
                        );

                        if let TextDecorationLine::Underline = style.text_decoration.line {
                            draw_line(
                                bbox.x + x,
                                bbox.y + y,
                                bbox.x + x + dimensions.width,
                                bbox.y + y,
                                1.0,
                                style.text_decoration.color.into(),
                            );
                        }

                        x += dimensions.width + space_width.width;
                    }
                }
                BoundingBox {
                    x: bbox.x,
                    y: bbox.y,
                    width: bbox.width,
                    height: y + line_height,
                }
            }
        },
        DOMElement::Text { text, style, .. } => {
            // Tokenization
            let tokens = text.split_whitespace();
            let mut x = 0.0;
            let line_height = style.font.size.to_pixels();
            let mut y = line_height;
            let space_width = measure_text(
                " ",
                Some(font),
                style.font.size.to_pixels().round() as u16,
                1.0,
            );

            for token in tokens {
                let dimensions = measure_text(token, Some(font), line_height.round() as u16, 1.0);

                if x + dimensions.width > bbox.width {
                    y += line_height;
                    x = 0.0;
                }

                draw_text(
                    &token,
                    bbox.x + x,
                    bbox.y + y,
                    line_height.round() as u16,
                    style.color.into(),
                );

                x += dimensions.width + space_width.width;
            }
            BoundingBox {
                x: bbox.x,
                y: bbox.y,
                width: bbox.width,
                height: y + line_height,
            }
        }
    }
}
