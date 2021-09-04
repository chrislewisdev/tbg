mod point;

use point::Point;
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
    fn get_cell(&self) -> Point;
    fn move_cells(&mut self, cells: Point);
    fn get_pixel(&self) -> Point;
    fn move_pixels(&mut self, pixels: Point);
}

struct Player {
    cell: Point,
    pixel: Point,
    get_cmd: fn (&dyn Unit, &InputState) -> Option<Box<dyn Cmd>>,
}

impl Unit for Player {
    fn get_cell(&self) -> Point { self.cell }
    fn move_cells(&mut self, cells: Point) { self.cell = self.cell + cells }
    fn get_pixel(&self) -> Point { self.pixel }
    fn move_pixels(&mut self, pixels: Point) { self.pixel = self.pixel + pixels }
}

struct GlobalState {
    player: Player,
}

trait Cmd {
    fn action(&mut self, unit: &mut dyn Unit);
    fn is_done(&self, unit: &dyn Unit) -> bool;
}

struct MoveCmd {
    target: Point,
}

impl Cmd for MoveCmd {
    fn action(&mut self, unit: &mut dyn Unit) {
        let target_pixel = self.target * TILE_SIZE;
        let unit_pixel = unit.get_pixel();
        let move_pixels = Point {
            x: i32::signum(target_pixel.x - unit_pixel.x),
            y: i32::signum(target_pixel.y - unit_pixel.y),
        };
        
        unit.move_pixels(move_pixels);
        if unit.get_pixel() == target_pixel {
            unit.move_cells(self.target - unit.get_cell());
        }
    }
    fn is_done(&self, unit: &dyn Unit) -> bool {
        return unit.get_cell() == self.target;
    }
}

fn player_control(unit: &dyn Unit, input_state: &InputState) -> Option<Box<dyn Cmd>> {
    if input_state.is_left_pressed {
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::LEFT }));
    } else if input_state.is_right_pressed {
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::RIGHT }));
    } else if input_state.is_up_pressed {
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::UP }));
    } else if input_state.is_down_pressed {
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::DOWN }));
    }

    return None;
}

trait ProgramState {
    fn update(&mut self, global_state: &mut GlobalState, input_state: &InputState);
    fn draw(&self, rl:&mut RaylibHandle, thread: &RaylibThread, resources: &Resources, global_state: &GlobalState);
}

struct ExploreState {
    cmd: Option<Box<dyn Cmd>>,
}

impl ProgramState for ExploreState {
    fn update(&mut self, global_state: &mut GlobalState, input_state: &InputState) {
        match &mut self.cmd {
            Some(cmd) => {
                if !cmd.is_done(&mut global_state.player) {
                    cmd.action(&mut global_state.player)
                }

                if cmd.is_done(&mut global_state.player) {
                    self.cmd = None
                }
            },
            None => {
                let get_cmd = global_state.player.get_cmd;

                self.cmd = get_cmd(&global_state.player, input_state);
            }
        }
    }
    
    fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread, resources: &Resources, global_state: &GlobalState) {
        let mut screen = rl.begin_drawing(&thread);
    
        screen.clear_background(Color::BLACK);
        screen.draw_texture_ex(&resources.room, Vector2::new(0.0, 0.0), 0.0, SCALE_FACTOR as f32, Color::WHITE);
    
        let x = global_state.player.pixel.x * SCALE_FACTOR;
        let y = global_state.player.pixel.y * SCALE_FACTOR;
        screen.draw_texture_ex(&resources.player_sprite, Vector2::new(x as f32, y as f32), 0.0, SCALE_FACTOR as f32, Color::WHITE);
    }
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

    let mut global_state = GlobalState { player: Player { cell: Point { x: 3, y: 3 }, pixel: Point { x: 3 * TILE_SIZE, y: 3 * TILE_SIZE }, get_cmd: player_control } };

    let mut explore_state = ExploreState { cmd: None };

    while !rl.window_should_close() {
        let input_state = InputState {
            is_left_pressed: rl.is_key_down(KeyboardKey::KEY_LEFT),
            is_right_pressed: rl.is_key_down(KeyboardKey::KEY_RIGHT),
            is_up_pressed: rl.is_key_down(KeyboardKey::KEY_UP),
            is_down_pressed: rl.is_key_down(KeyboardKey::KEY_DOWN),
        };

        explore_state.update(&mut global_state, &input_state);
        explore_state.draw(&mut rl, &thread, &resources, &global_state);
    }
}