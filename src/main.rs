extern crate libc;

use libc::{c_int};

#[link(name = "glfw3")]
extern {
	fn glfwInit() -> c_int;
}

fn main() {
	println!("Hello from rust-ffi-glfw!");

	unsafe {
		glfwInit();
	}

	println!("Glfw3 works!");
}