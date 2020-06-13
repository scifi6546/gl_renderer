use glutin::{self, PossiblyCurrent};

use std::ffi::CStr;
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
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
    let mut shader_program = 0;
    unsafe {
        let vertex_shader = gl.CreateShader(gl::VERTEX_SHADER);
        gl.ShaderSource(vertex_shader,1,[VS_SRC.as_ptr() as *const _].as_ptr(),std::ptr::null());
        gl.CompileShader(vertex_shader);
        {
            let mut sucess=0;
            gl.GetShaderiv(vertex_shader,gl::COMPILE_STATUS,&mut sucess as *mut i32);
            
            if sucess!=1{
                println!("compiliation failed!!!");

            }
        }
        let fragment_shader = gl.CreateShader(gl::FRAGMENT_SHADER);
        gl.ShaderSource(fragment_shader,1,[FS_SRC.as_ptr() as *const _].as_ptr(),std::ptr::null());
        gl.CompileShader(fragment_shader);
        {
            let mut sucess=1;
            gl.GetShaderiv(fragment_shader,gl::COMPILE_STATUS,&mut sucess as *mut i32);
            if sucess!=1{
                println!("compiliation failed!!!");

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
    pub fn draw_frame(&self, color: [f32; 4], verticies: Vec<f32>, indicies: Vec<u32>) {
       // println!("drawing color: {}", color[0]);
        unsafe {
            self.gl.ClearColor(color[0],color[1],color[2],color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.UseProgram(self.shader_program);
            self.gl.BindVertexArray(self.vertex_attribute_array);
            //binding data
            self.gl.BindBuffer(gl::ARRAY_BUFFER,self.vertex_buffer);
            self.gl.BufferData(gl::ARRAY_BUFFER,(verticies.len()*std::mem::size_of::<f32>()) as isize,verticies.as_ptr() as *const std::ffi::c_void,gl::DYNAMIC_DRAW);

            self.gl.BindBuffer(gl::ELEMENT_ARRAY_BUFFER,self.element_buffer_object);
            self.gl.BufferData(gl::ELEMENT_ARRAY_BUFFER,(indicies.len()*std::mem::size_of::<u32>()) as isize,indicies.as_ptr() as *const std::ffi::c_void,gl::DYNAMIC_DRAW);

            self.gl.VertexAttribPointer(0,3,gl::FLOAT,gl::FALSE,(3*std::mem::size_of::<f32>()) as i32,0 as *const std::ffi::c_void);
            self.gl.EnableVertexAttribArray(0);
            self.gl.DrawElements(gl::TRIANGLES,indicies.len() as i32,gl::UNSIGNED_INT,0 as *const _);
            self.gl.BindBuffer(gl::ARRAY_BUFFER,0);
            self.gl.BindVertexArray(0);
            let e = self.gl.GetError();
            if e!=0{
                println!("gl error: {}",e);

            }
                
            
        }
    }
}

const VS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

attribute vec3 position;
//attribute vec3 color;

varying vec3 v_color;

void main() {
    gl_Position = vec4(position, 1.0);
    v_color = vec3(1.0,0.5,1.0);
}
\0";

const FS_SRC: &'static [u8] = b"
#version 100
precision mediump float;

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
