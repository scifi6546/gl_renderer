mod support;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
pub fn init() {
    #[rustfmt::skip]
    let mut VERTEX_DATA:Vec<f32> = vec! [
     1.0,  0.0,  0.0,
     0.0,  1.0,  0.0,  
    -1.0,  0.0,  0.0,  
     0.0, -1.0,  0.0,  
    ];
    let indicies = vec![0, 2, 1, 0, 2, 3];
    let el = EventLoop::new();
    let wb = WindowBuilder::new().with_title("A fantastic window!");

    let windowed_context = ContextBuilder::new().build_windowed(wb, &el).unwrap();

    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    println!(
        "Pixel format of the window's GL context: {:?}",
        windowed_context.get_pixel_format()
    );

    let gl = support::load(&windowed_context.context());
    let mut color = 0.0;
    el.run(move |event, _, control_flow| {
        //println!("{:?}", event);
        //*control_flow = ControlFlow::Wait;
        //VERTEX_DATA[0]+=0.0001;
        //VERTEX_DATA[3]+=0.0001;
        //VERTEX_DATA[6]+=0.0001;
 //       println!("color: {}", color);
        color += 0.000123;
        if color > 1.0 {
            color = 0.0
        }
        gl.draw_frame(
            [color, color, color, 1.0],
            VERTEX_DATA.clone(),
            indicies.clone(),
        );
        windowed_context.swap_buffers().unwrap();
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => windowed_context.resize(physical_size),
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            Event::RedrawRequested(_) => {}
            _ => (),
        }
    });
}
