use gl_renderer::{init, run,Renderable,Model};
struct State{

}
impl Renderable for State{
    fn render(&mut self)->Vec<Model>{
        vec![]

    }

}
fn main() {
    println!("Hello, world!");
    run(||State{},init());
}
