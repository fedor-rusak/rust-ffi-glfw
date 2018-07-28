extern crate first_lib;

fn main() {
	first_lib::glfw3_opengl_helper::main();
	println!("Answer for evrything is {}!", first_lib::helper_old::answer_for_everything());
}