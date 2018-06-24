use std::ptr;
use std::ffi::{CString};

use std::os::raw::c_char;

extern crate libc;

use libc::{c_int, c_float, c_void, c_uint};

#[allow(missing_copy_implementations)]
pub enum GLFWmonitor {}

#[allow(missing_copy_implementations)]
pub enum GLFWwindow {}

#[allow(missing_copy_implementations)]
pub enum GLenum {}

pub static GL_COLOR_BUFFER_BIT: c_uint = 0x00004000; //it is a macro constant :(

#[link(name = "glfw3")]
extern {
	fn glfwInit() -> c_int;
	fn glfwPollEvents() -> c_int;
	fn glfwCreateWindow(width: c_int, height: c_int, title: *const c_char, monitor: *mut GLFWmonitor, share: *mut GLFWwindow) -> *mut GLFWwindow;
	fn glfwMakeContextCurrent(window: *mut GLFWwindow) -> c_void;
	fn glfwWindowShouldClose(window: *mut GLFWwindow) -> c_int;
	fn glfwSwapBuffers(window: *mut GLFWwindow) -> c_void;
	fn glfwSetWindowSizeCallback(window: *mut GLFWwindow, onResizeCallback: extern fn(window: *mut GLFWwindow, i32, i32)) -> c_void;
}

#[link(name = "glew32")]
extern "stdcall" { //this is some wicked mumbo-jumbo for windows macro and dll
	fn glewInit() -> c_int;
}

#[link(name = "OpenGL32")]
extern "stdcall" {
	fn glClearColor(r: c_float, g: c_float, b: c_float, a: c_float) -> c_void;
	fn glClear(bitmask: c_uint) -> c_void;
}


#[allow(unused_variables)]
extern fn on_resize_callback(window: *mut GLFWwindow, width: i32, height: i32) {
    println!("I'm called from C with value {0} and {1}", width, height);
}


fn main() {
	println!("Hello from rust-ffi-glfw!");

	unsafe {
		let string = CString::new("Something".as_bytes()).unwrap(); //tricky stuff. If written in one line string would vanish!
		let title = string.as_bytes_with_nul().as_ptr() as *const c_char;

		glfwInit();

		let window = glfwCreateWindow(800 as c_int, 600 as c_int, title, ptr::null_mut(), ptr::null_mut());

		glfwSetWindowSizeCallback(window, on_resize_callback);

		glfwMakeContextCurrent(window);

		println!("GLFW window was opened!");

		if glewInit() == 0 {
			println!("GLEW initialized!");
		}
		else {
			println!("GLEW failed to initialize!");
		}

		glClearColor(0.3, 0.4, 0.1, 1.0);

		loop {
			glfwPollEvents();

			if glfwWindowShouldClose(window) == 1 {
				break;
			}

			glClear(GL_COLOR_BUFFER_BIT);

			glfwSwapBuffers(window);
		};
	}

	println!("GLFW window was closed successfully!");
}