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

trait Unit {
    fn get_cell(&self) -> Point;
    fn move_cells(&mut self, cells: Point);
    fn get_pixel(&self) -> Point;
    fn move_pixels(&mut self, pixels: Point);
}

struct Player {
    cell: Point,
    pixel: Point,
    get_cmd: fn (&dyn Unit, &InputState, &GlobalState) -> Option<Box<dyn Cmd>>,
}

impl Unit for Player {
    fn get_cell(&self) -> Point { self.cell }
    fn move_cells(&mut self, cells: Point) { self.cell = self.cell + cells }
    fn get_pixel(&self) -> Point { self.pixel }
    fn move_pixels(&mut self, pixels: Point) { self.pixel = self.pixel + pixels }
}

type MapData = Vec<Vec<i32>>;

struct GlobalState {
    player: Player,
    map: MapData,
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
            x: i32::signum(target_pixel.x - unit_pixel.x) * 2,
            y: i32::signum(target_pixel.y - unit_pixel.y) * 2,
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

fn is_empty(map: &MapData, x: usize, y: usize) -> bool {
    return map[y][x] == 1;
}

fn player_control(unit: &dyn Unit, input_state: &InputState, global_state: &GlobalState) -> Option<Box<dyn Cmd>> {
    let cell = unit.get_cell();
    let x = cell.x as usize;
    let y = cell.y as usize;

    if input_state.is_left_pressed && is_empty(&global_state.map, x - 1, y) {
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::LEFT }));
    } else if input_state.is_right_pressed && is_empty(&global_state.map, x + 1, y){
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::RIGHT }));
    } else if input_state.is_up_pressed && is_empty(&global_state.map, x, y - 1) {
        return Some(Box::new(MoveCmd { target: unit.get_cell() + point::UP }));
    } else if input_state.is_down_pressed && is_empty(&global_state.map, x, y + 1) {
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
        // Get a new cmd if we have none or the current is done.
        if self.cmd.as_ref().map_or(true, |cmd| cmd.is_done(&global_state.player)) {
            let get_cmd = global_state.player.get_cmd;
            self.cmd = get_cmd(&global_state.player, input_state, global_state);
        }

        match &mut self.cmd {
            Some(cmd) => {
                cmd.action(&mut global_state.player);
            }
            _ => {}
        }
    }
    
    fn draw(&self, rl: &mut RaylibHandle, thread: &RaylibThread, resources: &Resources, global_state: &GlobalState) {
        let mut screen = rl.begin_drawing(&thread);
    
        screen.clear_background(Color::BLACK);
        // TODO: Render map data out
        // screen.draw_texture_ex(&resources.room, Vector2::new(0.0, 0.0), 0.0, SCALE_FACTOR as f32, Color::WHITE);
        draw_map(&mut screen, &global_state.map, &resources.tileset);

        let x = global_state.player.pixel.x * SCALE_FACTOR;
        let y = global_state.player.pixel.y * SCALE_FACTOR;
        screen.draw_texture_ex(&resources.player_sprite, Vector2::new(x as f32, y as f32), 0.0, SCALE_FACTOR as f32, Color::WHITE);
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

    Rectangle { x: (x * TILE_SIZE) as f32, y: (y * TILE_SIZE) as f32, width: TILE_SIZE as f32 , height: TILE_SIZE as f32 }
}

fn draw_map(screen: &mut RaylibDrawHandle, map: &MapData, tileset: &Texture2D) {
    let mut y = 0;

    for row in map {
        let mut x = 0;
        for tile in row {
            screen.draw_texture_pro(
                tileset,
                get_tile_rect(&tileset, *tile),
                Rectangle { x: (x * TILE_SIZE * SCALE_FACTOR) as f32, y: (y * TILE_SIZE * SCALE_FACTOR) as f32, width: (TILE_SIZE * SCALE_FACTOR) as f32, height: (TILE_SIZE * SCALE_FACTOR) as f32 },
                Vector2 { x: 0.0, y: 0.0 },
                0.0,
                Color::WHITE
            );
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
        player: Player {
            cell: Point { x: 3, y: 3 },
            pixel: Point { x: 3 * TILE_SIZE, y: 3 * TILE_SIZE },
            get_cmd: player_control
        },
        map
    };

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