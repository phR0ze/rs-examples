//! Demonstrating label basics
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "label".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let mut fps = Fps::new();
    loop {
        clear_background(WHITE);
        fps.show(&mut *root_ui(), None);

        Label::new("Test").with_layout(|x| x.with_align(Align::Center)).show(&mut *root_ui(), None);

        next_frame().await
    }
}
