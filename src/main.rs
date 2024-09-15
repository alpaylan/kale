use std::collections::HashMap;

use dom::DOM;
use macroquad;
use macroquad::prelude::*;
use rendering::render_dom;
use styling::{FontFamily, FontWeight};

mod dom;
mod html;
mod parser;
mod rendering;
mod styling;

#[macroquad::main("Kale")]
async fn main() {
    let html_elements = parser::parse(include_str!("../project2.html")).unwrap();
    let mut dom = DOM::construct_dom(html_elements);
    let mut fonts = HashMap::new();
    fonts.insert(
        (FontFamily::TimesNewRoman, FontWeight::Normal),
        load_ttf_font("tnr.ttf").await.unwrap(),
    );
    fonts.insert(
        (FontFamily::TimesNewRoman, FontWeight::Bold),
        load_ttf_font("tnrb.ttf").await.unwrap(),
    );

    let draw_text = |text: &str, x: f32, y: f32, font_size: u16, color: Color, font: &Font| {
        macroquad::text::draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(font),
                font_size: font_size,
                font_scale: 1.0,
                font_scale_aspect: 1.0,
                rotation: 0.0,
                color: color,
            },
        )
    };

    loop {
        let element_boxes = render_dom(&dom, &draw_text, &fonts);
        // if macroquad::input::is_mouse_button_down(macroquad::input::MouseButton::Left) {
        //     for (bbox, actions, id) in element_boxes {
        //         println!(
        //             "{id}: {:?}: {:?}",
        //             bbox,
        //             macroquad::input::mouse_position()
        //         );
        //         if bbox.contains(macroquad::input::mouse_position().into()) {
        //             println!("Clicked on {id}");
        //             for action in actions {
        //                 println!("{:?}", action);
        //                 match action {
        //                     dom::DOMAction::ClickToRedirect(url) => dom.set_clicked(&id),
        //                 }
        //             }
        //         }
        //     }
        // }

        let cursor = macroquad::input::mouse_position();
        for (bbox, actions, id) in element_boxes {
            if bbox.contains(cursor.into()) {
                dom.set_hovered(&id);
            }
        }
        next_frame().await
    }
}
