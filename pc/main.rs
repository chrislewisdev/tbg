use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 352;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .build();

    // Put the window in the corner (useful when watching)
    let monitor_width = raylib::core::window::get_monitor_width(0);
    rl.set_window_position(monitor_width - WINDOW_WIDTH, 40);

    let room = rl.load_texture(&thread, "room.png").expect("Failed to load room.png");
    let sprite = rl.load_texture(&thread, "sprite.png").expect("Failed to load sprite.png");

    while !rl.window_should_close() {
        let mut screen = rl.begin_drawing(&thread);

        screen.clear_background(Color::BLACK);
        screen.draw_texture_ex(&room, Vector2::new(0.0, 0.0), 0.0, 4.0, Color::WHITE);
        screen.draw_texture_ex(&sprite, Vector2::new(300.0, 150.0), 0.0, 4.0, Color::WHITE);
    }
}