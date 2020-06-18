use nalgebra::Vector3;
#[derive(Clone)]
pub struct Model{
    pub verticies:Vec<Vector3<f32>>,
    pub indicies:Vec<u32>,
}
