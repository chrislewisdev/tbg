use raylib::prelude::*;

const TILE_SIZE: i32 = 8;
const SCALE_FACTOR: i32 = 4;

const SCREEN_WIDTH_CELLS: i32 = 20;
const SCREEN_HEIGHT_CELLS: i32 = 11;

const WINDOW_WIDTH: i32 = SCREEN_WIDTH_CELLS * TILE_SIZE * SCALE_FACTOR;
const WINDOW_HEIGHT: i32 = SCREEN_HEIGHT_CELLS * TILE_SIZE * SCALE_FACTOR;

struct Resources {
    room: Texture2D,
    player_sprite: Texture2D,
}

struct InputState {
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
}

trait Unit {
    fn get_x(&self) -> i32;
    fn set_x(&mut self, x: i32);
    fn get_y(&self) -> i32;
    fn set_y(&mut self, y: i32);
}

struct Player {
    x: i32,
    y: i32,
    get_cmd: fn (&InputState) -> Option<Cmd>,
}

impl Unit for Player {
    fn get_x(&self) -> i32 { self.x }
    fn set_x(&mut self, x: i32) { self.x = x }
    fn get_y(&self) -> i32 { self.y }
    fn set_y(&mut self, y: i32) { self.y = y}
}

struct GlobalState {
    player: Player,
}

type Cmd = fn (&mut dyn Unit) -> bool;

fn cmd_move_left(unit: &mut dyn Unit) -> bool {
    unit.set_x(unit.get_x() - 1);
    return true;
}
fn cmd_move_right(unit: &mut dyn Unit) -> bool {
    unit.set_x(unit.get_x() + 1);
    return true;
}
fn cmd_move_up(unit: &mut dyn Unit) -> bool {
    unit.set_y(unit.get_y() - 1);
    return true;
}
fn cmd_move_down(unit: &mut dyn Unit) -> bool {
    unit.set_y(unit.get_y() + 1);
    return true;
}

fn player_control(input_state: &InputState) -> Option<Cmd> {
    if input_state.is_left_pressed {
        return Some(cmd_move_left);
    } else if input_state.is_right_pressed {
        return Some(cmd_move_right);
    } else if input_state.is_up_pressed {
        return Some(cmd_move_up);
    } else if input_state.is_down_pressed {
        return Some(cmd_move_down);
    }

    return None;
}

fn update_game(global_state: &mut GlobalState, input_state: &InputState) {
    let get_cmd = global_state.player.get_cmd;

    if let Some(cmd) = get_cmd(input_state) {
        cmd(&mut global_state.player);
    }
}

fn draw_game(rl: &mut RaylibHandle, thread: &RaylibThread, resources: &Resources, global_state: &GlobalState) {
    let mut screen = rl.begin_drawing(&thread);

    screen.clear_background(Color::BLACK);
    screen.draw_texture_ex(&resources.room, Vector2::new(0.0, 0.0), 0.0, SCALE_FACTOR as f32, Color::WHITE);

    let x = global_state.player.x * TILE_SIZE * SCALE_FACTOR;
    let y = global_state.player.y * TILE_SIZE * SCALE_FACTOR;
    screen.draw_texture_ex(&resources.player_sprite, Vector2::new(x as f32, y as f32), 0.0, SCALE_FACTOR as f32, Color::WHITE);
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Hello, World")
        .build();

    rl.set_target_fps(60);

    // Put the window in the corner (useful when watching)
    let monitor_width = raylib::core::window::get_monitor_width(0);
    rl.set_window_position(monitor_width - WINDOW_WIDTH, 40);

    let room = rl.load_texture(&thread, "room.png").expect("Failed to load room.png");
    let sprite = rl.load_texture(&thread, "sprite.png").expect("Failed to load sprite.png");
    let resources = Resources { room: room, player_sprite: sprite };

    let mut global_state = GlobalState { player: Player { x: 3, y: 3, get_cmd: player_control } };

    while !rl.window_should_close() {
        let input_state = InputState {
            is_left_pressed: rl.is_key_down(KeyboardKey::KEY_LEFT),
            is_right_pressed: rl.is_key_down(KeyboardKey::KEY_RIGHT),
            is_up_pressed: rl.is_key_down(KeyboardKey::KEY_UP),
            is_down_pressed: rl.is_key_down(KeyboardKey::KEY_DOWN),
        };

        update_game(&mut global_state, &input_state);
        draw_game(&mut rl, &thread, &resources, &global_state);
    }
}