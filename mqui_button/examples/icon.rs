use mqui_button::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "mqui_button".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    let icon = Image::from_file_with_format(include_bytes!("../assets/options_icon.png"), None);
    let mut button =
        Button::icon("Settings", icon).with_position(Position::LeftTop(None)).with_background_color(GRAY);

    loop {
        clear_background(BLACK);

        button.ui(&mut *root_ui(), Size::screen());
        if button.toggle() {
            draw_rectangle(100., 100., 50., 50., RED);
        }

        next_frame().await
    }
}
