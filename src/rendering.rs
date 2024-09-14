use macroquad::{color::Color, shapes::draw_line, text::TextDimensions};

use crate::dom::{DOMElement, Span, DOM};

pub fn render_dom(
    dom: &DOM,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
) {
    macroquad::window::clear_background(macroquad::color::WHITE);

    let mut y = 30.0;
    let mut m = 0.0;
    for element in dom.elements.iter() {
        (y, m) = render_dom_element(element, 0.0, y, m, draw_text);
    }
}

pub(crate) fn render_dom_element(
    element: &DOMElement,
    x: f32,
    y: f32,
    margin: f32,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
) -> (f32, f32) {
    let style = element.style();
    let mut y = y + f32::max(style.margin.top.to_pixels(), margin);
    match element {
        DOMElement::Paragraph(p) => {
            for span in p.spans.iter() {
                match span {
                    Span::Text(text) => {
                        let dimensions = draw_text(
                            text.as_str(),
                            x as f32,
                            y + style.margin.top.to_pixels(),
                            style.font.size.to_pixels().round() as u16,
                            style.color.into(),
                        );
                        y += dimensions.height
                    }
                    Span::Anchor(anchor) => {
                        let dimensions = draw_text(
                            anchor.text.as_str(),
                            x as f32,
                            y + style.margin.top.to_pixels(),
                            style.font.size.into(),
                            Color::from_rgba(0, 0, 255, 255),
                        );
                        y += dimensions.height;
                    }
                }
            }
            (y, style.margin.bottom.to_pixels())
        }
        DOMElement::Heading(h) => {
            let dimensions = draw_text(
                h.text.as_str(),
                x as f32,
                y,
                style.font.size.into(),
                style.color.into(),
            );
            (y + dimensions.height, style.margin.bottom.to_pixels())
        }
        DOMElement::Anchor(anchor) => {
            let dimensions = draw_text(
                anchor.text.as_str(),
                x as f32,
                y,
                style.font.size.into(),
                style.color.into(),
            );
            draw_line(x, y, x + dimensions.width, y, 1.0, style.color.into());
            (y + dimensions.height, style.margin.bottom.to_pixels())
        }
        DOMElement::DescriptionList(description_list) => todo!(),
    }
}
