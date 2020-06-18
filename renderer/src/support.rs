mod model;
pub use model::Model;
use glutin::{self, PossiblyCurrent};
use nalgebra::{Matrix4};
use std::ffi::CString;

use std::ffi::CStr;
pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
    vertex_buffer: u32,
    element_buffer_object:u32,
    vertex_attribute_array: u32,
    shader_program:u32,
}

pub fn load(gl_context: &glutin::Context<PossiblyCurrent>) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);
    let mut vertex_buffer = 0;
    let mut vertex_attribute_array = 0;
    let mut element_buffer_object = 0;
    #[allow(unused_assignments)]
    let mut shader_program = 0;
    unsafe {
        let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        gl.ShaderSource(vertex_shader,1,[VS_SRC.as_ptr() as *const _].as_ptr(),std::ptr::null());
        gl.CompileShader(vertex_shader);
        {
            let mut sucess=0;
            gl.GetShaderiv(vertex_shader,gl::COMPILE_STATUS,&mut sucess as *mut i32);
            
            if sucess!=1{
                let mut error_log = [0;512];
                let mut len = 0;
                gl.GetShaderInfoLog(vertex_shader,512,&mut len,error_log.as_mut_ptr());
                let error_str = String::from_raw_parts(error_log.as_mut_ptr() as *mut u8,len as usize,512);
                println!("vertex shader compiliation failed!!!");
                println!("{}",error_str);

            }
        }
        let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        gl.ShaderSource(fragment_shader,1,[FS_SRC.as_ptr() as *const _].as_ptr(),std::ptr::null());
        gl.CompileShader(fragment_shader);
        {
            let mut sucess=1;
            gl.GetShaderiv(fragment_shader,gl::COMPILE_STATUS,&mut sucess as *mut i32);
            if sucess!=1{
                println!("fragment shader compiliation failed!!!");

            }
        }
        shader_program = gl.CreateProgram();
        gl.AttachShader(shader_program,vertex_shader);
        gl.AttachShader(shader_program,fragment_shader);
        gl.LinkProgram(shader_program);
        {
            let mut sucess = 1;
            gl.GetProgramiv(shader_program,gl::LINK_STATUS,&mut sucess as *mut i32);
            if sucess!=1{
                println!("failed to link program!");

            }

        }

        gl.DeleteShader(vertex_shader);
        gl.DeleteShader(fragment_shader);

        gl.GenVertexArrays(1,&mut vertex_attribute_array);
        gl.GenBuffers(1,&mut vertex_buffer);
        gl.GenBuffers(1,&mut element_buffer_object);

        gl.BindVertexArray(vertex_attribute_array);



        

        
        gl.BindVertexArray(0);
    }

    Gl {
        gl: gl,
        vertex_buffer: vertex_buffer,
        element_buffer_object: element_buffer_object,
        vertex_attribute_array: vertex_attribute_array,
        shader_program:shader_program,
    }
}

impl Gl {
    unsafe fn get_error(&self){
        let e = self.gl.GetError();
        if e!=0{
            println!("gl error: {}",e);

        }
    }
    pub fn draw_frame(&self, color: [f32; 4], model: Vec<Model>) {
       // println!("drawing color: {}", color[0]);
        unsafe {
            self.gl.ClearColor(color[0],color[1],color[2],color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.UseProgram(self.shader_program);
            self.get_active_uniforms();
            self.get_error();

            for m in model.iter(){
                self.draw_model(m);
            }
        }
    }
    unsafe fn draw_model(&self,model: &Model){
            let m = nalgebra::one::<Matrix4<f32>>();
            println!("{}",m);
            let m_ptr = m.as_slice().as_ptr();
            println!("shader program: {}",self.shader_program);
            let model_location = self.gl.GetUniformLocation(self.shader_program,CString::new("model").expect("failed??").as_ptr() as *const i8);
            println!("model location {}",model_location);
            self.gl.UniformMatrix4fv(model_location,1,gl::FALSE,m_ptr);


            let m2 = nalgebra::one::<Matrix4<f32>>();

            let view_location = self.gl.GetUniformLocation(self.shader_program,CString::new("view").expect("failed??").as_ptr() as *const i8);
            println!("view location {}",view_location);
            self.gl.UniformMatrix4fv(view_location,1,gl::FALSE,m2.as_slice().as_ptr());

            let m3 = nalgebra::one::<Matrix4<f32>>();
            let position_location = self.gl.GetUniformLocation(self.shader_program,CString::new("position_mat").expect("failed??").as_ptr() as *const i8);
            println!("position location: {}",position_location);
            self.gl.UniformMatrix4fv(position_location,1,gl::FALSE,m3.as_slice().as_ptr());
            println!("{}",m);
            self.get_error();


            self.gl.BindVertexArray(self.vertex_attribute_array);
            //binding data
            self.gl.BindBuffer(gl::ARRAY_BUFFER,self.vertex_buffer);
            self.gl.BufferData(gl::ARRAY_BUFFER,(model.verticies.len()*3*std::mem::size_of::<f32>()) as isize,model.verticies.as_ptr() as *const std::ffi::c_void,gl::DYNAMIC_DRAW);

            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER,self.element_buffer_object);
            self.gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,(model.indicies.len()*std::mem::size_of::<u32>()) as isize,model.indicies.as_ptr() as *const std::ffi::c_void,gl::DYNAMIC_DRAW);

            self.gl.VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,(3*std::mem::size_of::<f32>()) as i32,0 as *const std::ffi::c_void);
            self.gl.EnableVertexAttribArray(0);
            self.gl.DrawElements(gl::TRIANGLES,model.indicies.len() as i32,gl::UNSIGNED_INT,0 as *const _);
            self.gl.BindBuffer(gl::ARRAY_BUFFER,0);
            self.gl.BindVertexArray(0);

    }
    unsafe fn get_active_uniforms(&self){
        let mut count = 0;
        self.gl.GetProgramiv(self.shader_program,gl::ACTIVE_UNIFORMS,&mut count);
        println!("num active uniforms: {}",count);
        for i in 0..count{
            let mut name_buff:Vec<u8> = vec![0;512];
            let mut type_buff = vec![0;512];
            let mut len = 0;
            let mut size = 0;
            self.gl.GetActiveUniform(self.shader_program,i as u32,511,&mut len,&mut size,type_buff.as_mut_ptr(),name_buff.as_mut_ptr() as *mut i8);
            let name_str = std::string::String::from_utf8(name_buff.clone()).ok().unwrap();
         //   let type_str = std::string::String::from_raw_parts(type_buff.as_mut_ptr() as *mut u8,512,512);
            println!("name: {} ",name_str);
            println!("{}",name_buff.iter().filter(|x| x!=&&0).map(|x| format!("{:#x}, ",x)).fold("".to_string(),|s,c| s+&c));

            //println!("name raw: {:?}",name_buff);

        }

    }
}

const VS_SRC: &'static [u8] = b"
#version 330 core
precision mediump float;

attribute vec3 position;
uniform mat4 model;
uniform mat4 view;
uniform mat4 position_mat;

varying vec3 v_color;

void main() {
    gl_Position = model*view*position_mat*vec4(position, 1.0);
    v_color = vec3(1.0,0.5,1.0);
}
\0";

const FS_SRC: &'static [u8] = b"
#version 330 core
precision mediump float;

uniform mat4 model;
uniform mat4 view;
uniform mat4 position_mat;

varying vec3 v_color;

void main() {
    gl_FragColor = vec4(v_color, 1.0);
}
\0";

pub use self::context_tracker::{ContextCurrentWrapper, ContextId, ContextTracker, ContextWrapper};

#[allow(dead_code)] // Not used by all examples
mod context_tracker {
    use glutin::{
        self, Context, ContextCurrentState, ContextError, NotCurrent, PossiblyCurrent,
        WindowedContext,
    };
    use takeable_option::Takeable;

    pub enum ContextWrapper<T: ContextCurrentState> {
        Headless(Context<T>),
        Windowed(WindowedContext<T>),
    }

    impl<T: ContextCurrentState> ContextWrapper<T> {
        pub fn headless(&mut self) -> &mut Context<T> {
            match self {
                ContextWrapper::Headless(ref mut ctx) => ctx,
                _ => panic!(),
            }
        }

        pub fn windowed(&mut self) -> &mut WindowedContext<T> {
            match self {
                ContextWrapper::Windowed(ref mut ctx) => ctx,
                _ => panic!(),
            }
        }

        fn map<T2: ContextCurrentState, FH, FW>(
            self,
            fh: FH,
            fw: FW,
        ) -> Result<ContextWrapper<T2>, (Self, ContextError)>
        where
            FH: FnOnce(Context<T>) -> Result<Context<T2>, (Context<T>, ContextError)>,
            FW: FnOnce(
                WindowedContext<T>,
            )
                -> Result<WindowedContext<T2>, (WindowedContext<T>, ContextError)>,
        {
            match self {
                ContextWrapper::Headless(ctx) => match fh(ctx) {
                    Ok(ctx) => Ok(ContextWrapper::Headless(ctx)),
                    Err((ctx, err)) => Err((ContextWrapper::Headless(ctx), err)),
                },
                ContextWrapper::Windowed(ctx) => match fw(ctx) {
                    Ok(ctx) => Ok(ContextWrapper::Windowed(ctx)),
                    Err((ctx, err)) => Err((ContextWrapper::Windowed(ctx), err)),
                },
            }
        }
    }

    pub enum ContextCurrentWrapper {
        PossiblyCurrent(ContextWrapper<PossiblyCurrent>),
        NotCurrent(ContextWrapper<NotCurrent>),
    }

    impl ContextCurrentWrapper {
        fn map_possibly<F>(self, f: F) -> Result<Self, (Self, ContextError)>
        where
            F: FnOnce(
                ContextWrapper<PossiblyCurrent>,
            ) -> Result<
                ContextWrapper<NotCurrent>,
                (ContextWrapper<PossiblyCurrent>, ContextError),
            >,
        {
            match self {
                ret @ ContextCurrentWrapper::NotCurrent(_) => Ok(ret),
                ContextCurrentWrapper::PossiblyCurrent(ctx) => match f(ctx) {
                    Ok(ctx) => Ok(ContextCurrentWrapper::NotCurrent(ctx)),
                    Err((ctx, err)) => Err((ContextCurrentWrapper::PossiblyCurrent(ctx), err)),
                },
            }
        }

        fn map_not<F>(self, f: F) -> Result<Self, (Self, ContextError)>
        where
            F: FnOnce(
                ContextWrapper<NotCurrent>,
            ) -> Result<
                ContextWrapper<PossiblyCurrent>,
                (ContextWrapper<NotCurrent>, ContextError),
            >,
        {
            match self {
                ret @ ContextCurrentWrapper::PossiblyCurrent(_) => Ok(ret),
                ContextCurrentWrapper::NotCurrent(ctx) => match f(ctx) {
                    Ok(ctx) => Ok(ContextCurrentWrapper::PossiblyCurrent(ctx)),
                    Err((ctx, err)) => Err((ContextCurrentWrapper::NotCurrent(ctx), err)),
                },
            }
        }
    }

    pub type ContextId = usize;
    #[derive(Default)]
    pub struct ContextTracker {
        current: Option<ContextId>,
        others: Vec<(ContextId, Takeable<ContextCurrentWrapper>)>,
        next_id: ContextId,
    }

    impl ContextTracker {
        pub fn insert(&mut self, ctx: ContextCurrentWrapper) -> ContextId {
            let id = self.next_id;
            self.next_id += 1;

            if let ContextCurrentWrapper::PossiblyCurrent(_) = ctx {
                if let Some(old_current) = self.current {
                    unsafe {
                        self.modify(old_current, |ctx| {
                            ctx.map_possibly(|ctx| {
                                ctx.map(
                                    |ctx| Ok(ctx.treat_as_not_current()),
                                    |ctx| Ok(ctx.treat_as_not_current()),
                                )
                            })
                        })
                        .unwrap()
                    }
                }
                self.current = Some(id);
            }

            self.others.push((id, Takeable::new(ctx)));
            id
        }

        pub fn remove(&mut self, id: ContextId) -> ContextCurrentWrapper {
            if Some(id) == self.current {
                self.current.take();
            }

            let this_index = self
                .others
                .binary_search_by(|(sid, _)| sid.cmp(&id))
                .unwrap();
            Takeable::take(&mut self.others.remove(this_index).1)
        }

        fn modify<F>(&mut self, id: ContextId, f: F) -> Result<(), ContextError>
        where
            F: FnOnce(
                ContextCurrentWrapper,
            )
                -> Result<ContextCurrentWrapper, (ContextCurrentWrapper, ContextError)>,
        {
            let this_index = self
                .others
                .binary_search_by(|(sid, _)| sid.cmp(&id))
                .unwrap();

            let this_context = Takeable::take(&mut self.others[this_index].1);

            match f(this_context) {
                Err((ctx, err)) => {
                    self.others[this_index].1 = Takeable::new(ctx);
                    Err(err)
                }
                Ok(ctx) => {
                    self.others[this_index].1 = Takeable::new(ctx);
                    Ok(())
                }
            }
        }

        pub fn get_current(
            &mut self,
            id: ContextId,
        ) -> Result<&mut ContextWrapper<PossiblyCurrent>, ContextError> {
            unsafe {
                let this_index = self
                    .others
                    .binary_search_by(|(sid, _)| sid.cmp(&id))
                    .unwrap();
                if Some(id) != self.current {
                    let old_current = self.current.take();

                    if let Err(err) = self.modify(id, |ctx| {
                        ctx.map_not(|ctx| {
                            ctx.map(|ctx| ctx.make_current(), |ctx| ctx.make_current())
                        })
                    }) {
                        // Oh noes, something went wrong
                        // Let's at least make sure that no context is current.
                        if let Some(old_current) = old_current {
                            if let Err(err2) = self.modify(old_current, |ctx| {
                                ctx.map_possibly(|ctx| {
                                    ctx.map(
                                        |ctx| ctx.make_not_current(),
                                        |ctx| ctx.make_not_current(),
                                    )
                                })
                            }) {
                                panic!(
                                    "Could not `make_current` nor `make_not_current`, {:?}, {:?}",
                                    err, err2
                                );
                            }
                        }

                        if let Err(err2) = self.modify(id, |ctx| {
                            ctx.map_possibly(|ctx| {
                                ctx.map(|ctx| ctx.make_not_current(), |ctx| ctx.make_not_current())
                            })
                        }) {
                            panic!(
                                "Could not `make_current` nor `make_not_current`, {:?}, {:?}",
                                err, err2
                            );
                        }

                        return Err(err);
                    }

                    self.current = Some(id);

                    if let Some(old_current) = old_current {
                        self.modify(old_current, |ctx| {
                            ctx.map_possibly(|ctx| {
                                ctx.map(
                                    |ctx| Ok(ctx.treat_as_not_current()),
                                    |ctx| Ok(ctx.treat_as_not_current()),
                                )
                            })
                        })
                        .unwrap();
                    }
                }

                match *self.others[this_index].1 {
                    ContextCurrentWrapper::PossiblyCurrent(ref mut ctx) => Ok(ctx),
                    ContextCurrentWrapper::NotCurrent(_) => panic!(),
                }
            }
        }
    }
}
