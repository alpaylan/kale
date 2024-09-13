use macroquad::prelude::*;

mod html;
mod parser;

#[macroquad::main("Kale")]
async fn main() {

    let htmlElements = parser::parse(include_str!("../project.html"));

    println!("{}", htmlElements[0].to_string());


    loop {
        clear_background(WHITE);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 20.0, DARKGRAY);

        next_frame().await
    }
}