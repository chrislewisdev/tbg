use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 352)
        .title("Hello, World")
        .build();

    let texture = rl.load_texture(&thread, "sprite.png").expect("");

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        d.draw_texture_ex(&texture, Vector2::new(300.0, 150.0), 0.0, 3.0, Color::WHITE);
    }
}