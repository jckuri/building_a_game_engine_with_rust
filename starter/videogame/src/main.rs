extern crate reqwest;
extern crate serde;
extern crate rand;

use std::ffi::CString;
use std::thread;
use std::time;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use std::fmt;
use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct Sprite {
 x: u16,
 y: u16,
 width: u16,
 height: u16,
 r: u8,
 g: u8,
 b: u8
}

impl fmt::Display for Sprite {
 fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
  write!(f, "Sprite {{ x: {}, y: {}, width: {}, height: {}, r: {}, g: {}, b: {} }}", self.x, self.y, self.width, self.height, self.r, self.g, self.b)
 }
}

fn fetch_url(url: &str) -> Result<String, reqwest::Error> {
 let response = blocking::get(url)?;
 let text = response.text()?;
 return Ok(text);
}

fn fetch_sprite() -> Result<Sprite, Box<dyn std::error::Error>> {
 let url = "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";
 let json_code: &str = &fetch_url(url)?;
 let sprite: Sprite = serde_json::from_str(json_code)?;
 return Ok(sprite);
}

fn sprites_loop(sprite_sender: std::sync::mpsc::Sender<Sprite>, end_receiver: std::sync::mpsc::Receiver<bool>) {
 loop {
  let result = fetch_sprite();
  match result {
   Ok(sprite) => {let _ = sprite_sender.send(sprite);}
   Err(error) => {println!("Error: {}", error);}
  }
  if let Ok(_) = end_receiver.try_recv() {   
   println!("End message received!");
   break;
  }
 }
}

fn rand0n(n: i32) -> u16 {
 return rand::thread_rng().gen_range(0..n + 1) as u16;
}

fn draw_sprites(sprites: &Vec<Sprite>) {
 for sprite in sprites {
  let r = rand0n(4);
  opengl_game::spawn_sprite!((sprite.x - r) as f32, (sprite.y - r) as f32, (sprite.width + 2 * r) as i32, (sprite.height + 2 * r) as i32, sprite.r as i32, sprite.g as i32, sprite.b as i32);
 }
}

fn main() {
 let (sprite_sender, sprite_receiver) = std::sync::mpsc::channel::<Sprite>();
 let (end_sender, end_receiver) = std::sync::mpsc::channel::<bool>();
 let networking_thread = std::thread::spawn(|| {sprites_loop(sprite_sender, end_receiver);});
 let title = CString::new("Rust Videogame").unwrap();
 let mut sprite_count = 0;
 let mut queue: Vec<Sprite> = Vec::new();
 opengl_game::start_window_and_game_loop!(title.as_ptr(), 800, 600, {
  opengl_game::opengl_ffi::rust_clear_screen(); 
  draw_sprites(&queue);  
  if let Ok(sprite) = sprite_receiver.try_recv() {
   sprite_count += 1;
   println!("{}: {}", sprite_count, sprite);
   queue.push(sprite);
   if queue.len() > 4 {queue.remove(0);}
  }
  if opengl_game::opengl_ffi::rust_window_should_close() == 1 || 
   opengl_game::opengl_ffi::rust_get_key(opengl_game::opengl_ffi::rust_get_window(), opengl_game::opengl_ffi::GLFW_KEY_ESCAPE) == opengl_game::opengl_ffi::GLFW_PRESS {
   let _ = end_sender.send(true);
   break;
  }
  opengl_game::tick!(10);
 });
 let _ = networking_thread.join();
}
