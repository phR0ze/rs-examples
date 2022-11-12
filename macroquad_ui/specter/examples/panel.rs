//! Demonstrating panel basics
use specter::prelude::*;

fn main_conf() -> Conf {
    Conf {
        window_title: "panel".to_string(),
        window_width: 450,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(main_conf)]
async fn main() {
    loop {
        clear_background(WHITE);

        let mut p1 = Panel::new("p1")
            .with_layout(|x| {
                x.with_mode(Mode::TopToBottom).with_spacing(10.).with_size_full().with_padding_all(30.)
            })
            .with_frame(|x| x.with_fill(BLACK));
        let mut r1 = Panel::new("p2")
            .with_layout(|x| {
                x.with_mode(Mode::LeftToRight)
                    .with_size_static(390., 120.0)
                    .with_spacing(10.)
                    .with_padding_all(20.)
                    .with_parent(&p1.layout())
            })
            .with_frame(|x| x.with_fill(DARKGRAY));
        let mut r1c1 = Panel::new("0")
            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.).with_parent(&r1.layout()))
            .with_frame(|x| x.with_fill(RED));
        let mut r1c2 = Panel::new("1")
            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.).with_parent(&r1.layout()))
            .with_frame(|x| x.with_fill(GRAY));
        let mut r1c3 = Panel::new("2")
            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.).with_parent(&r1.layout()))
            .with_frame(|x| x.with_fill(BLUE));

        let mut r2 = Panel::new("p3")
            .with_layout(|x| {
                x.with_mode(Mode::LeftToRight)
                    .with_size_static(390., 200.0)
                    .with_spacing(10.)
                    .with_padding_all(20.)
                    .with_parent(&p1.layout())
            })
            .with_frame(|x| x.with_fill(GREEN));
        let mut r2c1 = Panel::new("0")
            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.).with_parent(&r2.layout()))
            .with_frame(|x| x.with_fill(RED));
        let mut r2c2 = Panel::new("1")
            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.).with_parent(&r2.layout()))
            .with_frame(|x| x.with_fill(GRAY));
        let mut r2c3 = Panel::new("2")
            .with_layout(|x| x.with_size_static(100., 100.).with_margins_all(5.).with_parent(&r2.layout()))
            .with_frame(|x| x.with_fill(BLUE));

        p1.show(&mut *root_ui());
        r1.show(&mut *root_ui());
        r1c1.show(&mut *root_ui());
        r1c2.show(&mut *root_ui());
        r1c3.show(&mut *root_ui());
        r2.show(&mut *root_ui());
        r2c1.show(&mut *root_ui());
        r2c2.show(&mut *root_ui());
        r2c3.show(&mut *root_ui());

        next_frame().await
    }
}
