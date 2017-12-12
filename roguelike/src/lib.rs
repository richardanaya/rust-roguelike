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
    view_width: i32,
    view_height: i32,
    player_position:(i32,i32)
}

//we can't have mutable statics so we need this mutex that holds it
//using a macro from a crate lazy_static
lazy_static! {
    static ref WORLD: Mutex<World> = Mutex::new(
        World {
            view_width: 0,
            view_height: 0,
            player_position : (1,1)
        }
    );
}

//a simple safe wrapper around calling the JS function
fn draw_character(x: i32,y: i32, char: u8,r: u8,g: u8,b: u8) -> () {
    unsafe {
        put_character(x,y,char as i32,r as i32,g as i32,b as i32);
    }
}

fn draw_world(world:&World){
    //draw grass
    for y in 0..world.view_height {
        for x in 0..world.view_width {
          draw_character(x,y,46,0,255,0);
        }
    }

    //draw player
    draw_character(world.player_position.0,world.player_position.1,64,255,255,255);
}

#[no_mangle]
pub fn start(width: i32, height: i32) -> () {
    //get the world, this gets unlocked on deallocation
    let world = &mut WORLD.lock().unwrap();
    world.view_width = width;
    world.view_height = height;
    draw_world(world);
}

#[no_mangle]
pub fn key_down(c: i32) -> () {
    //get the world, this gets unlocked on deallocation
    let world = &mut WORLD.lock().unwrap();
    let modifier = match c {
        KEY_LEFT => (-1,0),
        KEY_RIGHT => (1,0),
        KEY_DOWN => (0,1),
        KEY_UP => (0,-1),
        _ => (0,0)
    };
    world.player_position = (world.player_position.0+modifier.0,world.player_position.1+modifier.1);
    draw_world(world);
}
