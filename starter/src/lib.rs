extern crate libc;

pub mod opengl_ffi {

 pub const GLFW_PRESS: libc::c_int = 1;
 pub const GLFW_RELEASE: libc::c_int = 0;
 pub const GLFW_KEY_ESCAPE: libc::c_int = 256;
 pub const GLFW_KEY_SPACE: libc::c_int = 32;
 pub const GLFW_KEY_RIGHT: libc::c_int = 262;
 pub const GLFW_KEY_LEFT: libc::c_int = 263;
 pub const GLFW_KEY_DOWN: libc::c_int = 264;
 pub const GLFW_KEY_UP: libc::c_int = 265;

 #[repr(C)]
 #[derive(Debug, Clone, Copy)]
 pub struct Sprite {
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub color: [libc::c_int; 3],
    pub x: libc::c_float,
    pub y: libc::c_float
 }
 
 // INSPIRED BY: Generating Rust FFI Bindings to C Libraries
 // https://users.rust-lang.org/t/generating-rust-ffi-bindings-to-c-libraries/18500
 #[repr(C)]
 #[derive(Debug, Clone, Copy)]
 pub struct GLFWwindow {
    _unused: [u8; 0]
 }

 extern "C" {
 
  fn create_game_window(title: *const libc::c_char, width: libc::c_int, height: libc::c_int);
  
  fn create_sprite(x: libc::c_float, y: libc::c_float, width: libc::c_int, height: libc::c_int, r: libc::c_int, g: libc::c_int, b: libc::c_int) -> *mut Sprite;
  
  fn render_sprite(Sprite: &mut Sprite);
  
  fn update_sprite_position(sprite: &mut Sprite, x: libc::c_float, y: libc::c_float);
  
  fn update_game_window();
  
  fn clear_screen();
  
  fn window_should_close() -> libc::c_int;

  fn get_key(window: *mut GLFWwindow, key: libc::c_int) -> libc::c_int;

  fn get_window() -> *mut GLFWwindow; 
  
 }

 pub fn rust_create_game_window(title: *const libc::c_char, width: libc::c_int, height: libc::c_int) {
  unsafe {
   create_game_window(title, width, height);
  }
 }
 
 pub fn rust_create_sprite(x: libc::c_float, y: libc::c_float, width: libc::c_int, height: libc::c_int, r: libc::c_int, g: libc::c_int, b: libc::c_int) -> Sprite {
  unsafe {
   return *create_sprite(x, y, width, height, r, g, b);
  }
 }
 
 pub fn rust_render_sprite(sprite: &mut Sprite) {
  unsafe {
   render_sprite(sprite);
  }
 }
 
 pub fn rust_update_sprite_position(sprite: &mut Sprite, x: libc::c_float, y: libc::c_float) {
  unsafe {
   update_sprite_position(sprite, x, y);
  }
 }
 
 pub fn rust_update_game_window() {
  unsafe {
   update_game_window();
  }
 }
 
 pub fn rust_clear_screen() {
  unsafe {
   clear_screen();
  }
 }
 
 pub fn rust_window_should_close() -> libc::c_int {
  unsafe {
   return window_should_close();
  }
 }
 
 pub fn rust_get_key(window: *mut GLFWwindow, key: libc::c_int) -> libc::c_int {
  unsafe {
   return get_key(window, key);
  }
 }
 
 pub fn rust_get_window() -> *mut GLFWwindow {
  unsafe {
   return get_window();
  }
 }
 
}


#[macro_export]
macro_rules! start_window_and_game_loop {
 ($title: expr, $width: expr, $height: expr, $loop_code: block) => {
  opengl_game::opengl_ffi::rust_create_game_window($title, $width, $height);
  loop {
   $loop_code
  }
 };
}

#[macro_export]
macro_rules! on_key_press {
 ($key: expr, $action: block) => {
  if opengl_game::opengl_ffi::rust_get_key(opengl_game::opengl_ffi::rust_get_window(), $key) == opengl_game::opengl_ffi::GLFW_PRESS
   $action
 };
}

#[macro_export]
macro_rules! on_key_release {
 ($key: expr, $action: block) => {
  if opengl_game::opengl_ffi::rust_get_key(opengl_game::opengl_ffi::rust_get_window(), $key) == opengl_game::opengl_ffi::GLFW_RELEASE
   $action
 };
}

#[macro_export]
macro_rules! spawn_sprite {
 ($x: expr, $y: expr, $width: expr, $height: expr, $r: expr, $g: expr, $b: expr) => {
  let mut sprite = opengl_game::opengl_ffi::rust_create_sprite($x, $y, $width, $height, $r, $g, $b);
  opengl_game::opengl_ffi::rust_render_sprite(&mut sprite);
 };
}

#[macro_export]
macro_rules! tick {
 ($milliseconds: expr) => {
  opengl_game::opengl_ffi::rust_update_game_window();
  thread::sleep(time::Duration::from_millis($milliseconds));
 };
}

#[macro_export]
macro_rules! conditional_break {
 ($condition: expr) => {
  if opengl_game::opengl_ffi::rust_window_should_close() == 1 {break;}
  if $condition {break;}
 };
 () => {
  if opengl_game::opengl_ffi::rust_window_should_close() == 1 {break;}
 };
}
