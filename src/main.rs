use dom::DOM;
use macroquad as mqd;
use macroquad::prelude::*;
use rendering::render_dom;

mod dom;
mod html;
mod parser;
mod rendering;
mod styling;

#[macroquad::main("Kale")]
async fn main() {
    let html_elements = parser::parse(include_str!("../project.html")).unwrap();
    let dom = DOM::construct_dom(html_elements);

    // mqd::window::set_fullscreen(true);
    let font = load_ttf_font("tnr.ttf").await.unwrap();

    let draw_text = |text: &str, x: f32, y: f32, font_size: u16, color: Color| {
        macroquad::text::draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(&font),
                font_size: font_size,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: color,
            },
        )
    };

    loop {
        render_dom(&dom, &draw_text, &font);
        next_frame().await
    }
}
