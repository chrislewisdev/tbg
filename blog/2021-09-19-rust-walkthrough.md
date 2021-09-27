# Rust Code Walkthrough

Well now that the basic movement code is done, let's take a look at how the code evolved from the Lua implementation! I'll start with the Rust version. This is my first time using Rust in any kind of project, though I have spent some time previously tinkering with it and reading some of the guides available on the Rust website. For this reason, I generally tried to keep the implementation on the simple side without using complex features of Rust if I could avoid it.

Since I needed a graphics library for this implementation, I ended up going with [raylib-rs](https://github.com/deltaphc/raylib-rs), a set of Rust bindings for raylib. This is also my first time using raylib, but I understand it to be a fairly popular C game development library. There may be some better alternatives out there - after all, these bindings are not fully complete and mainly cover the core framework - but I decided it would be sufficient for my purposes and didn't want to get stuck for too long just deciding on a library.

Anyway, let's jump into the code! If you haven't read the [Lua code walkthrough](2021-09-09-lua-walkthrough.md), it would probably be good to do so as I don't intend to re-explain some of the design concepts that I covered in that post.

## Core

Any good program starts with a `main` method, so let's start there.

```rust
use raylib::prelude::*;

const TILE_SIZE: i32 = 8;
const SCALE_FACTOR: i32 = 4;

const SCREEN_WIDTH_CELLS: i32 = 20;
const SCREEN_HEIGHT_CELLS: i32 = 11;

const WINDOW_WIDTH: i32 = SCREEN_WIDTH_CELLS * TILE_SIZE * SCALE_FACTOR;
const WINDOW_HEIGHT: i32 = SCREEN_HEIGHT_CELLS * TILE_SIZE * SCALE_FACTOR;

fn main() {
  let (mut rl, thread) = raylib::init()
    .size(WINDOW_WIDTH, WINDOW_HEIGHT)
    .title("tbg")
    .build();

  rl.set_target_fps(60);

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
```

As you'd expect, this is mainly about setting up our window and loading/instantiating the data we need to manage the game state. One decision I had to make early on was what size to make the game display at. Obviously, on desktop platforms, I have a much higher display resolution available over tha paltry 160x144 display of the Gameboy, or PICO-8's 128x128. However, to help maintain consistency between platforms, and also in mercy to *incredibly* poor art skills, I still wanted to keep the resolution quite low, and just scale it up to a sensible viewing size - hence, the various constants defined at the top of the screen defining how big the window should be in grid cells, and the factor to scale it up by. This gives me a resolution of 160x88 pixels to play with - creating a widescreen display rather than a square-ish one. I also made this choice so that if I add a fullscreen option in the future, it can scale fairly closely to the full screen size.

*(I might go back on this decision in the future if it becomes easier to make the display size match the other platforms more closely. Designing the game's UI to work in a widescreen format coming from the other sizes might prove awkward.)*

Anyway, the meat of the game of course is in the `update` and `draw` methods used in the main game loop here. Let's move on to those.

```rust
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
```

As in the Lua implementation, I've set up a basic concept of program states to be responsible for exploration, battle, menu states etc. Here I've codified the interface for a state as a Rust `trait` so it should hopefully be easy to utilise different states in a uniform manner later on. The `ExploreState` which is responsible for letting the player move around the dungeon implement a basic command pattern, again like the Lua version - only this time we are using function pointers and Rust's `Option` features to accomplish it (since Rust has no concept of null). It took me a few iterations of working out how to use Rust's `Option` in a sensible manner to get to this fairly simple implementation, but I like how it looks at the moment.

The `draw` method should be largely self-explanatory, but I will go into more detail on the `draw_map` implementation which I had to write a bit further down. I also needed to make sure that everything drawn gets positioned according to the game's scale factor, since player/enemy co-ordinates will be expressed in unscaled co-ordinates, for simplicity.

## Player control

So far, things are pretty similar to the Lua implementation, just considerably more verbose. This is largely the case for the player control methods as well:

```rust
type Cmd = fn (&mut Unit) -> bool;

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
```

Yep, the structure looks largely identical here. The struct holding the player's information will store a pointer to `player_control` function, and the update loop will use it to obtain its commands. To make coding these commands a little easier I introduced a `Point` struct that holds x/y co-ordinates instead of constantly passing x/y as separate variables, keeping the code more brief and readable. The `Point` struct supports basic addition/subtraction and any other mathematical operations that I happened to need:

```rust
use std::ops;

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

pub const UP: Point = Point { x: 0, y: -1 };
pub const DOWN: Point = Point { x: 0, y: 1 };
pub const LEFT: Point = Point { x: -1, y: 0 };
pub const RIGHT: Point = Point { x: 1, y: 0 };

impl ops::Add<Point> for Point {
  type Output = Point;

  fn add(self, rhs: Point) -> Point {
    Point { x: self.x + rhs.x, y: self.y + rhs.y }
  }
}

impl ops::Sub<Point> for Point {
  type Output = Point;

  fn sub(self, rhs: Point) -> Point {
    Point { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}

impl ops::Mul<i32> for Point {
  type Output = Point;

  fn mul(self, rhs: i32) -> Point {
    Point { x: self.x * rhs, y: self.y * rhs }
  }
}

```

There is probably some existing structs, either as part of raylib or some Rust crate somewhere, that would provide this common functionality for me, but implementing it myself at least allowed me to familiarise myself with Rust's operator overloading features. Note the up/down/left/right constants there to provide some convenience too.

## Tilemap functionality

As I've mentioned before, one of the key differences between this implementation and the others is that there is no native tilemap support I could find in raylib, so I had to add a basic implementation of tilemaps myself. This is taken care of by the following functions:

```rust
type MapData = Vec<Vec<i32>>;

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
```

The tilemap data is represented as a 2D array of tile indices, each denoting which tile in the tileset should be drawn at that position. I create the tilemap in [Tiled](https://www.mapeditor.org/), export it to CSV, and then load the CSV data in `load_map`. I must admit I cobbled the `load_map` method together from Rust's documentation examples without yet having a super-solid grasp on how Rust's iterators work, but it is a good starting point for now.

The `draw_map` function, naturally, loops through all the x/y grid co-ordinates represented by the tilemap and draws each one to screen. Rather than drawing a single texture to screen though, the tile we need to draw is only one part of the larger tileset image, so we use `get_tile_rect` to calculate what area of the tileset needs to be drawn according to the ID of the tile. Then `draw_tile` wraps up the myriad arguments we need to pass to raylib in order to draw that one tile to exactly where we want it.

I have a feeling that this `draw_map` implementation might not be terribly efficient, since we are making a lot of separate draw calls, when it could be better to first pre-render the tilemap out to its own texture and then draw that in a single call (so the entire tilemap does not need to be re-calculated each frame). However, without an in-depth knowledge of how raylib's draw functions work, and without a large enough tilemap for performance to matter right now, I will simply accept that I don't know any better right now and that I can revisit it only if necessary later on.

## Regarding traits

I'd like to note that while my type definition for a Command here is simply an alias for a function pointer (`type Cmd = fn (&mut Unit) -> bool;`) I initially attempted to make it a little more flexible using Rust's traits system. This looked something like this:

```rust
trait Cmd {
  fn action(&self, &mut Unit);
  fn is_done(&self);
}
```

In doing so I hoped that I could decouple the performing of the command from the *checking* of whether or not it was completed, and make the intent of the design a little clearer. In a language such as C# where this would be declared as an interface, this would be a common and easy way to pass around commands that all share that interface. In Rust, however, while this does allow us to write the code in a slightly more generic manner, the matter is made a bit more complex by the language's strict rules around types and memory allocation.

By opting for a trait rather than a simple function pointer, I would need to mark most functions that accept `Cmd` as an argument with `dyn Cmd` instead, and when instantiating a command, I would need to return it via a `Box` rather than an outright value so that the memory allocation for it is always constant. Although in practice I think these need not be too much of a barrier, it started to get a bit too complex for my novice Rust skills to cleanly manage the usage of traits in this generic fashion, and for very little benefit might I add. So, I opted to return to passing around straight-up function pointers instead.

*(I'll also note that I messed around with using closures rather than function pointers, but since function pointers and closures in Rust cannot quite be used interchangeably, this too started to boggle me. But I definitely learned some useful info about the intricacies of Rust in the process)*

## Conclusion

Well, that brings us to the end already! I have omitted some small pieces of code such as type definitions which I feel do not serve a great purpose in explaining the code right now, but that should give you a fairly complete picture of the code in its current state. It is considerably longer than the PICO-8 implementation, as expected, but still not very large as of yet. It was fun to finally put my beginner Rust skills to practice to bring this together too; I cannot put my finger on why exactly, but I do appreciate how the Rust implementation "looks".

Now, let's see when I can bring myself to try do a walkthrough for the most verbose implementation yet: the Gameboy assembly. I might need to cut down the code samples to just what is most pertinent rather than showing *all* the code in order to make things manageable, but we will see!