extern crate render_lib;

fn main() {
	render_lib::opengl_renderer::main();
	println!("Answer for evrything is {}!", render_lib::helper_old::answer_for_everything());
}