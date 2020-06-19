use gl_renderer::{init, run, Model, Renderable};
use nalgebra::Vector3;
#[macro_use]
extern crate ecs;
mod planets{
    use gl_renderer::Model;
    use nalgebra::Vector3;
    create_entity!(mass:f64,position:Vector3<f64>,velocity:Vector3<f64>,draw:Model); 
}
struct State {
    planet_system:planets::EntityManager,
}
impl State{
    pub fn new()->Self{
        let mut s = State{
            planet_system:planets::EntityManager::new(),
        };
        s.planet_system.new_entity(planets::Entity::new(||{Some(1.0)},||{Some(Vector3::new(0.0,0.0,0.0))},||{Some(Vector3::new(0.0,0.0,0.0))},||None,vec![]));
        return s;

    }

}

impl Renderable for State {
    fn render(&mut self) -> Vec<Model> {
        self.planet_system.process();
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
    run(|| State::new(), init());
}
