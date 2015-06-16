extern crate glium;

fn main() {
    use glium::DisplayBuild;

    let builder = glium::glutin::WindowBuilder::new();

    let _display = builder.build_glium();
}
