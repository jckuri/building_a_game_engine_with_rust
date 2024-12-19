#[cfg(test)]
mod tests {

 use std::ffi::CString;
 use std::thread;
 use std::time;

 #[test]
 fn test_simple_game_loop() {
  let title = CString::new("test_simple_game_loop").unwrap();
  opengl_game::start_window_and_game_loop!(title.as_ptr(), 400, 300, {
   opengl_game::opengl_ffi::rust_clear_screen();   
   opengl_game::tick!(10);
   opengl_game::conditional_break!();
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, {break;});
  });
 }
 
 #[test]
 fn test_sprite_rendering() {
  let (d1, d2) = (10, 50); 
  let title = CString::new("test_sprite_rendering").unwrap();
  opengl_game::start_window_and_game_loop!(title.as_ptr(), 400, 300, {
   opengl_game::opengl_ffi::rust_clear_screen();   
   opengl_game::spawn_sprite!(d1 as f32, d1 as f32, 400 - 2 * d1, 300 - 2 * d1, 255, 0, 0);
   opengl_game::spawn_sprite!(d2 as f32, d2 as f32, 400 - 2 * d2, 300 - 2 * d2, 0, 0, 255);
   opengl_game::tick!(10);
   opengl_game::conditional_break!();
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, {break;});
  });
 }
 
 #[test]
 fn test_screen_clearing() {
  let (d1, d2) = (10, 50); 
  let mut mode = 1;
  let mut count = 0;
  let title = CString::new("test_screen_clearing").unwrap();
  opengl_game::start_window_and_game_loop!(title.as_ptr(), 400, 300, {
   opengl_game::opengl_ffi::rust_clear_screen();   
   if mode % 4 == 1 {
    opengl_game::spawn_sprite!(d1 as f32, d1 as f32, 400 - 2 * d1, 300 - 2 * d1, 255, 0, 0);
   }
   if mode % 4 == 3 {
    opengl_game::spawn_sprite!(d2 as f32, d2 as f32, 400 - 2 * d2, 300 - 2 * d2, 0, 0, 255);
   }
   count = count + 1;
   if count % 20 == 0 {mode = mode + 1;}
   opengl_game::tick!(10);
   opengl_game::conditional_break!();
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, {break;});
  });
 }
 
 #[test]
 fn test_key_presses() {
  let mut intensity;
  let mut left = false;
  let mut down = false;
  let mut right = false;
  let mut up = false;
  let title = CString::new("test_key_presses - Press the arrow keys!").unwrap();
  opengl_game::start_window_and_game_loop!(title.as_ptr(), 500, 300, {
   opengl_game::opengl_ffi::rust_clear_screen();   
   if left {intensity = 255;} else {intensity = 128;}
   opengl_game::spawn_sprite!(100.0, 200.0, 90, 90, intensity, intensity, intensity);
   if down {intensity = 255;} else {intensity = 128;}
   opengl_game::spawn_sprite!(200.0, 200.0, 90, 90, intensity, intensity, intensity);
   if right {intensity = 255;} else {intensity = 128;}
   opengl_game::spawn_sprite!(300.0, 200.0, 90, 90, intensity, intensity, intensity);
   if up {intensity = 255;} else {intensity = 128;}
   opengl_game::spawn_sprite!(200.0, 100.0, 90, 90, intensity, intensity, intensity);   
   opengl_game::tick!(10);
   opengl_game::conditional_break!();
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, {break;});
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_LEFT, {left = true;});
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_DOWN, {down = true;});
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_RIGHT, {right = true;});
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_UP, {up = true;});
   opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_LEFT, {left = false;});
   opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_DOWN, {down = false;});
   opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_RIGHT, {right = false;});
   opengl_game::on_key_release!(opengl_game::opengl_ffi::GLFW_KEY_UP, {up = false;});
  });
 }
  
 #[test]
 fn test_sprite_position_update() {
  let mut x = 0.0;
  let mut y = 0.0;
  let mut dx = 1.0;
  let mut dy = 1.0;
  let title = CString::new("test_sprite_position_update").unwrap();
  let mut sprite = opengl_game::opengl_ffi::rust_create_sprite(x, y, 150, 100, 255, 0, 0);
  opengl_game::start_window_and_game_loop!(title.as_ptr(), 400, 300, {
   opengl_game::opengl_ffi::rust_clear_screen();
   opengl_game::opengl_ffi::rust_render_sprite(&mut sprite);
   x = x + dx;
   y = y + dy;
   if x > 400.0 - 150.0 {dx = -1.0;}
   if y > 300.0 - 100.0 {dy = -1.0;}
   if x < 0.0 {dx = 1.0;}
   if y < 0.0 {dy = 1.0;}
   opengl_game::opengl_ffi::rust_update_sprite_position(& mut sprite, x, y);
   opengl_game::tick!(10);
   opengl_game::conditional_break!();
   opengl_game::on_key_press!(opengl_game::opengl_ffi::GLFW_KEY_ESCAPE, {break;});
  });
 }
 
}
