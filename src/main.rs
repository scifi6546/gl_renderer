use gl_renderer::{init, run, Model, Renderable};
use nalgebra::Vector3;
struct State {}
impl Renderable for State {
    fn render(&mut self) -> Vec<Model> {
        vec![Model {
            verticies: vec![
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
                Vector3::new(0.0, -1.0, 0.0),
            ],
            indicies: vec![0, 2, 1, 0, 2, 3],
        }]
    }
}
fn main() {
    println!("Hello, world!");
    run(|| State {}, init());
}
