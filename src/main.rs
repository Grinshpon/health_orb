extern crate sdl2;

pub mod entity;
use crate::entity::*;

use std::process;
use std::path::Path;
use std::time::Duration;
use std::collections::HashMap;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use sdl2::EventPump;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;
use sdl2::render::{Texture, Canvas};
use sdl2::rect::Rect;
use sdl2::video::Window;

macro_rules! texture {
  ( $tx:expr, $p:expr ) => {{
    let tmp_surface: Surface = LoadSurface::from_file($p)
      .expect("Error loading image");
    $tx.create_texture_from_surface(&tmp_surface)
      .expect("Error creating texture")
  }}
}
macro_rules! hashmap {
  ( $( $key: expr => $val:expr ),* ) => {{
    let mut temp = HashMap::new();
    $(
      temp.insert($key,$val);
    )*
    temp
  }}
}
macro_rules! get {
  ( $map:expr, $key: expr ) => {(
    $map.get_mut($key).unwrap()
  )}
}

fn main() {
  init();
}

fn init() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let window = video_subsystem.window("health orb demo", 800, 600)
      .position_centered()
      .build()
      .unwrap();
  let mut canvas = window.into_canvas().build().unwrap();
  let mut event_pump = sdl_context.event_pump().unwrap();
  run(&mut canvas, &mut event_pump);
}

fn draw_entities(canvas: &mut Canvas<Window>, entities: &HashMap<&str, Entity>, textures: &HashMap<&str, Texture>) {
  let pool: Vec<&Entity> = {
    let mut temp: Vec<(&&str, &Entity)> = entities.iter().collect();
    temp.sort_by(|(_,va),(_,vb)| vb.z.cmp(&va.z));
    temp.into_iter().map(|(_,e)| e).rev().collect() // There's definitely a better way to do this.
    //temp
  };
  for entity in pool.iter() {
    canvas.copy(&textures[entity.img],Some(entity.src),Some(entity.dst()))
      .expect("Error rendering textures");
  }
}

fn run(canvas: &mut Canvas<Window>, event_pump: &mut EventPump) {
  let texture_creator = canvas.texture_creator();
  let mut textures = hashmap!
    { "texture_orb" => texture!(texture_creator,Path::new("assets/globe.png"))
    , "texture_bar" => texture!(texture_creator,Path::new("assets/globefill.png"))
    };
  let mut entities = hashmap!
    { "bar" => Entity
      ( "texture_bar"
      , 9
      , Rect::new(0,0,64,64)
      , Rect::new(10,0,512,512)
      )
    , "orb" => Entity
      ( "texture_orb"
      , 10
      , Rect::new(0,0,64,64)
      , Rect::new(10,0,512,512)
      )
    };

  let mut hp: u32 = 98;
  let mut fhp: f32 = 64.0 * ((hp as f32)/100.0);

  canvas.set_draw_color(Color::RGB(140, 140, 140));
  canvas.clear();
  canvas.present();
  let mut update = || {
    if fhp > 31.0 { //min visibility at 4, 1 down on 96, fhp: 3.0 - 62.0
      hp -= 1;
      fhp = 64.0 * (hp as f32/100.0);
      get!(entities,"bar").src.y = 64 - fhp as i32;
      get!(entities,"bar").src.set_height(fhp as u32);
      get!(entities,"bar").setY(512 - (fhp as i32 * 8));
      get!(entities,"bar").set_height(fhp as u32 * 8);
    }
    canvas.clear();
    event_handler(event_pump);

    // The rest of the game loop goes here...
    draw_entities(canvas, &entities, &textures);
    canvas.present();
    //temporary frame limiter
    std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
  };
  loop {update();}
}

fn event_handler(event_pump: &mut EventPump) {
  for event in event_pump.poll_iter() {
    match event {
      Event::Quit {..} |
      Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
        process::exit(1);
      },
      _ => {}
    }
  }
}
