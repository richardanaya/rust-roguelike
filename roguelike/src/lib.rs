//using special macros for global state, see below
#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;

//this represents our external js function
extern {
    fn put_character(x: i32,y: i32,char: i32,r: i32,g: i32,b: i32);
}

const KEY_LEFT: i32 = 37;
const KEY_UP: i32 = 38;
const KEY_RIGHT: i32 = 39;
const KEY_DOWN: i32 = 40;

struct World {
    width: i32,
    height: i32,
    player_position:(i32,i32)
}

//we can't have mutable statics so we need this mutex that holds it
//using a macro from a crate lazy_static
lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(
        World {
            width: 0,
            height: 0,
            player_position : (5,5)
        }
    );
}

//a simple safe wrapper around calling the JS function
fn draw_character(x: i32,y: i32, char: u8,r: u8,g: u8,b: u8) -> () {
    unsafe {
        put_character(x,y,char as i32,r as i32,g as i32,b as i32);
    }
}

fn draw_world(){
    //get reference to world data
    let world = &WORLD.lock().unwrap();

    //draw grass
    for y in 0..world.height {
        for x in 0..world.width {
          draw_character(x,y,46,0,255,0);
        }
    }

    //draw player
    draw_character(world.player_position.0,world.player_position.1,64,255,255,255);
}

#[no_mangle]
pub fn start(width: i32, height: i32) -> () {
    let world = &mut WORLD.lock().unwrap();
    world.width = width;
    world.height = height;
    draw_world();
}

#[no_mangle]
pub fn key_down(c: i32) -> () {
    let world = &mut WORLD.lock().unwrap();
    if(c == KEY_LEFT){
        world.player_position = (world.player_position.0-1,world.player_position.1);
    }
    draw_world();
}
