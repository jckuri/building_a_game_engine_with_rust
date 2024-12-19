extern crate cc;

fn main() {
 cc::Build::new()
  .file("opengl_wrapper_lib/opengl_wrapper_lib.c")
  .compile("libopengl_wrapper_lib.a");
  
 println!("cargo::rustc-link-lib=glfw");
 println!("cargo::rustc-link-lib=GL");
}
