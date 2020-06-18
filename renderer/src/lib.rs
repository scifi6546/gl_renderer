mod support;
pub use support::Model;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
pub trait Renderable{
    fn render(&mut self)->Vec<Model>;
}
pub struct GraphicsState{
    gl:support::Gl,
    event_loop:EventLoop<()>,
    window_context:glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>
}
pub fn init()->GraphicsState {
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };
    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let gl = support::load(&windowed_context.context());
    return GraphicsState{
        gl:gl,
        event_loop:el,
        window_context:windowed_context,

    }
}
pub fn run<State:Renderable+'static>(state:fn()->State,gl:GraphicsState){
    _run(state,gl.gl,gl.event_loop,gl.window_context);

}
fn _run<State:Renderable+'static>(
    state_factory:fn()->State, 
    gl:support::Gl,
    event_loop:EventLoop<()>,
    window_context:glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>){
    let mut color = 0.0;
    #[rustfmt::skip]
    let mut state = state_factory();
    event_loop.run(move |event, _, control_flow| {
        let models = state.render();
        color += 0.000123;
        if color > 1.0 {
            color = 0.0
        }
        gl.draw_frame(
            [color, color, color, 1.0],
            models
        );
        window_context.swap_buffers().unwrap();
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => window_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
