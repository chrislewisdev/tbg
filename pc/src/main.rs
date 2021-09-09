mod point;

use std::fs;
use point::Point;
use raylib::prelude::*;

const TILE_SIZE: i32 = 8;
const SCALE_FACTOR: i32 = 4;

const SCREEN_WIDTH_CELLS: i32 = 20;
const SCREEN_HEIGHT_CELLS: i32 = 11;

const WINDOW_WIDTH: i32 = SCREEN_WIDTH_CELLS * TILE_SIZE * SCALE_FACTOR;
const WINDOW_HEIGHT: i32 = SCREEN_HEIGHT_CELLS * TILE_SIZE * SCALE_FACTOR;

type MapData = Vec<Vec<i32>>;
type Cmd = fn (&mut Unit) -> bool;

struct Resources {
    player_sprite: Texture2D,
    tileset: Texture2D,
}

struct InputState {
    is_left_pressed: bool,
    is_right_pressed: bool,
    is_up_pressed: bool,
    is_down_pressed: bool,
}

struct Unit {
    cell: Point,
    pixel: Point,
    get_cmd: fn (&Unit, &InputState, &GlobalState) -> Option<Cmd>,
}

struct GlobalState {
    player: Unit,
    map: MapData,
}

fn move_cmd(unit: &mut Unit, delta: Point) -> bool {
    let target_pixel = (unit.cell + delta) * TILE_SIZE;
    let move_pixels = Point {
        x: i32::signum(target_pixel.x - unit.pixel.x) * 2,
        y: i32::signum(target_pixel.y - unit.pixel.y) * 2,
    };
    
    unit.pixel = unit.pixel + move_pixels;
    if unit.pixel == target_pixel {
        unit.cell = unit.cell + delta;
        return true;
    }

    return false;
}
fn move_left_cmd(unit: &mut Unit) -> bool { move_cmd(unit, point::LEFT) }
fn move_right_cmd(unit: &mut Unit) -> bool { move_cmd(unit, point::RIGHT) }
fn move_up_cmd(unit: &mut Unit) -> bool { move_cmd(unit, point::UP) }
fn move_down_cmd(unit: &mut Unit) -> bool { move_cmd(unit, point::DOWN) }

fn is_empty(map: &MapData, x: i32, y: i32) -> bool {
    return map[y as usize][x as usize] == 1;
}

fn player_control(unit: &Unit, input_state: &InputState, global_state: &GlobalState) -> Option<Cmd> {
    if input_state.is_left_pressed && is_empty(&global_state.map, unit.cell.x - 1, unit.cell.y) {
        return Some(move_left_cmd);
    } else if input_state.is_right_pressed && is_empty(&global_state.map, unit.cell.x + 1, unit.cell.y){
        return Some(move_right_cmd);
    } else if input_state.is_up_pressed && is_empty(&global_state.map, unit.cell.x, unit.cell.y - 1) {
        return Some(move_up_cmd);
    } else if input_state.is_down_pressed && is_empty(&global_state.map, unit.cell.x, unit.cell.y + 1) {
        return Some(move_down_cmd);
    }

    return None;
}

trait ProgramState {
    fn update(&mut self, global_state: &mut GlobalState, input_state: &InputState);
    fn draw(&self, screen: &mut RaylibDrawHandle, resources: &Resources, global_state: &GlobalState);
}

struct ExploreState {
    cmd: Option<Cmd>,
}

impl ProgramState for ExploreState {
    fn update(&mut self, global_state: &mut GlobalState, input_state: &InputState) {
        if self.cmd.is_none() {
            let get_cmd = global_state.player.get_cmd;
            self.cmd = get_cmd(&global_state.player, input_state, global_state);
        }

        if let Some(cmd) = self.cmd {
            if cmd(&mut global_state.player) {
                self.cmd = None
            }
        }
    }
    
    fn draw(&self, screen: &mut RaylibDrawHandle, resources: &Resources, global_state: &GlobalState) {
        screen.clear_background(Color::BLACK);
        draw_map(screen, &global_state.map, &resources.tileset);

        let x = global_state.player.pixel.x * SCALE_FACTOR;
        let y = global_state.player.pixel.y * SCALE_FACTOR;
        screen.draw_texture_ex(&resources.player_sprite, rvec2(x, y), 0.0, SCALE_FACTOR as f32, Color::WHITE);
    }
}

fn load_map() -> MapData {
    let mut output: MapData = vec![];
    let csv = fs::read_to_string("assets/map.csv").expect("Failed to load map.csv");
    let mut reader = csv::ReaderBuilder::new().has_headers(false).from_reader(csv.as_bytes());

    for result in reader.records() {
        let row = result.unwrap();
        output.push(row.iter().map(|s| s.parse::<i32>().unwrap()).collect());
    }

    return output;
}

fn get_tile_rect(tileset: &Texture2D, index: i32) -> Rectangle {
    let tiles_width = tileset.width / TILE_SIZE;
    let x = index % tiles_width;
    let y = index / tiles_width;

    rrect(x * TILE_SIZE, y * TILE_SIZE, TILE_SIZE, TILE_SIZE)
}

fn draw_tile(screen: &mut RaylibDrawHandle, tileset: &Texture2D, tile: i32, x: i32, y: i32) {
    screen.draw_texture_pro(
        tileset,
        get_tile_rect(&tileset, tile),
        rrect(x * TILE_SIZE * SCALE_FACTOR, y * TILE_SIZE * SCALE_FACTOR, TILE_SIZE * SCALE_FACTOR, TILE_SIZE * SCALE_FACTOR),
        rvec2(0, 0),
        0.0,
        Color::WHITE
    );
}

fn draw_map(screen: &mut RaylibDrawHandle, map: &MapData, tileset: &Texture2D) {
    let mut y = 0;

    for row in map {
        let mut x = 0;
        for tile in row {
            draw_tile(screen, tileset, *tile, x, y);
            x += 1;
        }
        y += 1;
    }
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("tbg")
        .build();

    rl.set_target_fps(60);

    // Put the window in the corner (useful when watching)
    let monitor_width = raylib::core::window::get_monitor_width(0);
    rl.set_window_position(monitor_width - WINDOW_WIDTH, 40);

    let sprite = rl.load_texture(&thread, "assets/sprite.png").expect("Failed to load sprite.png");
    let tileset = rl.load_texture(&thread, "assets/tileset.png").expect("Failed to load tileset.png");
    let resources = Resources { player_sprite: sprite, tileset };

    let map = load_map();

    let mut global_state = GlobalState {
        player: Unit {
            cell: Point { x: 3, y: 3 },
            pixel: Point { x: 3 * TILE_SIZE, y: 3 * TILE_SIZE },
            get_cmd: player_control
        },
        map
    };

    let mut explore_state = ExploreState { cmd: None };

    while !rl.window_should_close() {
        let input_state = InputState {
            is_left_pressed: rl.is_key_pressed(KeyboardKey::KEY_LEFT),
            is_right_pressed: rl.is_key_pressed(KeyboardKey::KEY_RIGHT),
            is_up_pressed: rl.is_key_pressed(KeyboardKey::KEY_UP),
            is_down_pressed: rl.is_key_pressed(KeyboardKey::KEY_DOWN),
        };

        explore_state.update(&mut global_state, &input_state);
        let mut screen = rl.begin_drawing(&thread);
        explore_state.draw(&mut screen, &resources, &global_state);
    }
}