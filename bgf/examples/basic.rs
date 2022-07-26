use bgf::{graphics::colors, Window};

fn main() {
    let mut window = Window::new(1280, 720, "Basic Example");
    let mut renderer = window.build_renderer(1);

    let can_pooper_texture = renderer.load_texture("assets/textures/can_pooper.png");

    window.show();

    while window.is_open() {
        renderer.begin();

        renderer.draw_quad(
            200.0,
            200.0,
            100.0,
            100.0,
            &colors::WHITE,
            can_pooper_texture,
        );

        renderer.end();

        window.update();
    }
}
