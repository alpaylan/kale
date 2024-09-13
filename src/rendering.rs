use macroquad::text::TextDimensions;

use crate::dom::{DOMElement, Span, DOM};

pub fn render_dom(
    dom: &DOM,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
) {
    macroquad::window::clear_background(macroquad::color::WHITE);

    let mut y = 30.0;
    for element in dom.elements.iter() {
        y = render_dom_element(element, 0.0, y, draw_text);
    }
}

pub(crate) fn render_dom_element(
    element: &DOMElement,
    x: f32,
    y: f32,
    draw_text: &dyn Fn(&str, f32, f32, u16, macroquad::color::Color) -> TextDimensions,
) -> f32 {
    let style = element.style();
    match element {
        DOMElement::Paragraph(p) => todo!(),
        DOMElement::Heading(h) => {
            println!("Rendering heading: {}", h.text);
            println!("Style: {:?}", style);
            let dimensions = draw_text(
                h.text.as_str(),
                x as f32,
                y + style.margin.top.to_pixels(),
                style.font.size.into(),
                style.color.into(),
            );
            y + dimensions.height + style.margin.bottom.to_pixels()
        }
        DOMElement::Anchor(anchor) => todo!(),
        DOMElement::DescriptionList(description_list) => todo!(),
    }
}
