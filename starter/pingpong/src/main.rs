use std::ffi::CString;
use std::thread;
use std::time;

fn main() {
 let screen_width = 800;
 let screen_height = 600;
 let thick = 20;
 let pad = 100;
 let left_pad_x: f32 = (thick * 2) as f32;
 let mut left_pad_y: f32 = (screen_height / 2 - pad / 2) as f32;
 let right_pad_x: f32 = (screen_width - thick * 3) as f32;
 let mut right_pad_y: f32 = (screen_height / 2) as f32;
 let mut x = 0.0;
 let mut y = 0.0;
 let speed = 2.0;
 let mut dx = speed;
 let mut dy = speed;
 let mut up = false;
 let mut down = false;
 let title = CString::new("Ping Pong - Press UP and DOWN to play").unwrap();
 let mut upper_wall = opengl_game::opengl_ffi::rust_create_sprite(0.0, 0.0, screen_width, thick, 255, 255, 255);
 let mut lower_wall = opengl_game::opengl_ffi::rust_create_sprite(0.0, (screen_height - thick) as f32, screen_width, thick, 255, 255, 255);
 let mut left_pad = opengl_game::opengl_ffi::rust_create_sprite(left_pad_x, left_pad_y, thick, pad, 255, 0, 0);
 let mut right_pad = opengl_game::opengl_ffi::rust_create_sprite(right_pad_x, right_pad_y, thick, pad, 255, 255, 255);
 let mut ball = opengl_game::opengl_ffi::rust_create_sprite(x, y, thick, thick, 255, 255, 255);
 x = (3 * thick) as f32;
 y = left_pad_y + (pad / 2 - thick / 2) as f32;
 opengl_game::start_window_and_game_loop!(title.as_ptr(), screen_width, screen_height, {
  opengl_game::opengl_ffi::rust_clear_screen();
  opengl_game::opengl_ffi::rust_render_sprite(&mut ball);
  opengl_game::opengl_ffi::rust_render_sprite(&mut upper_wall);
  opengl_game::opengl_ffi::rust_render_sprite(&mut lower_wall);
  opengl_game::opengl_ffi::rust_render_sprite(&mut left_pad);
  opengl_game::opengl_ffi::rust_render_sprite(&mut right_pad);
  x = x + dx;
  y = y + dy;
  if x > (screen_width - 4 * thick) as f32 {dx = -speed;}
  if y > (screen_height - 2 * thick) as f32 {dy = -speed;}
  if y < thick as f32 {dy = speed;}
  if x > (2 * thick) as f32 && x < (3 * thick) as f32 {
   if (y + thick as f32) > left_pad_y && y < (left_pad_y + pad as f32) {dx = speed;}
  }  
  if x < (-10 * thick) as f32 {
   x = (3 * thick) as f32;
   y = left_pad_y + (pad / 2 - thick / 2) as f32;
   dx = speed;
  }
  right_pad_y = y - (pad / 2 - thick / 2) as f32;
  if right_pad_y < thick as f32 {right_pad_y = thick as f32;}
  if right_pad_y > (screen_height - thick - pad) as f32 {right_pad_y = (screen_height - thick - pad) as f32;}
  opengl_game::opengl_ffi::rust_update_sprite_position(&mut ball, x, y);
  opengl_game::opengl_ffi::rust_update_sprite_position(&mut right_pad, right_pad_x, right_pad_y);
  opengl_game::opengl_ffi::rust_update_sprite_position(&mut left_pad, left_pad_x, left_pad_y);
  opengl_game::tick!(10);
  opengl_game::conditional_break!();
  opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, {break;});
  opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_DOWN, {down = true;});
  opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_UP, {up = true;});
  opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_DOWN, {down = false;});
  opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_UP, {up = false;});
  if up {left_pad_y -= speed;}
  if down {left_pad_y += speed;}
  if left_pad_y < thick as f32 {left_pad_y = thick as f32;}
  if left_pad_y > (screen_height - thick - pad) as f32 {left_pad_y = (screen_height - thick - pad) as f32;}
 });
}
