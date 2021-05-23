use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::{ event::*};
use futures;

use crate::graphics;

///this is a function that takes no parameters and returns nothing, that can be used to get around
///rust limits on inherientance that I would normally use to build app window. Basically you pass
/// operation you want to happen on certain events here sort of like an observer/subscriber pattern. 
/// I might turn this into that in the future.
pub type CallOut = Box<dyn Fn() -> ()>;
pub type TimedCallOut = Box<dyn Fn(std::time::Duration) -> ()>;
pub type EventCallOut = Box<dyn Fn(&DeviceEvent) -> ()>;

pub struct AppWindow{
    pub state: graphics::State,
    pub window: winit::window::Window,
    //For now this can live here.
    camera: crate::camera::CameraObject,
    camera_gpu_object: crate::camera::GPUObject<crate::camera::Uniforms>,
    mouse_pressed: bool,

    exit_call: Option<CallOut>,
    resize_call: Option<CallOut>,
    update_call: Option<TimedCallOut>,
    input_call: Option<EventCallOut>,
}

impl AppWindow{
    pub fn new(width: u32, height: u32, event_loop: &EventLoop<()>) -> Self{
        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            WindowBuilder::new()
                .with_title("Hello World")
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
            };
        
        //Maybe have appwindow also return a &window and let the engine place the graphics state?
        //or leave the window with a single graphics state? 
        use futures::executor::block_on;
        let mut state = block_on(graphics::State::new(&window));

        //While it might be easier to build the camera first and then pass it while building the state I think the
        //API will work better if you set up the camera and other gpu object afterwards.
        let uniform_bind_group_layout = state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let mut camera = crate::camera::CameraObject::new(&state.sc_desc);
        camera.update();
        let camera_gpu_object = crate::camera::GPUObject::new(&state.device, &uniform_bind_group_layout, camera.uniforms);
        //END CAMERA CODE

        //Init the render Pipeline here for now.
        state.init_pipeline(&uniform_bind_group_layout);

        return Self{
            state,
            window,
            camera,
            camera_gpu_object,
            mouse_pressed: false,
            exit_call: None,
            resize_call: None,
            update_call: None,
            input_call: None,
        }
    }

    pub fn set_call_backs(&mut self, exit_call: CallOut, resize_call: CallOut,
    update_call: TimedCallOut){
        self.exit_call = Some(exit_call);
        self.resize_call = Some(resize_call);
        self.update_call = Some(update_call);
        //self.input_call = Some(input_call);
    }

    pub fn init_graphics_device(&mut self, bind_group_layout: &wgpu::BindGroupLayout){
        self.state.init_pipeline(&bind_group_layout);
    }

    pub fn run(event_loop: EventLoop<()>, mut app: Self, game: impl CallBack + 'static ) -> !
    {
        let exit = || game.exit();
        let resize = || game.resize();
        let update = |dt| game.update(dt);
        let update = |event| game.input(event);
       
        let mut input = WinitInputHelper::new();
        let mut last_render_time = std::time::Instant::now();

        event_loop.run(move |event, _, mut control_flow| {
            //So I am just going to update input here at the beginning of the loop and then pass
            //it to the application when the time comes.
            input.update(&event);
            if input.key_pressed(VirtualKeyCode::Escape) {*control_flow = ControlFlow::Exit;}
            
            match event {
                //Event main events are cleared with request a redraw?
                Event::MainEventsCleared => app.window.request_redraw(),
                //I am not handling events or device Id here, 
                Event::DeviceEvent { ref event, ..} => {
                    //doubling up while we sort this code out.
                    app.on_input(event, &input);
                }
                //Handle window specific events and other things winit picks up I guess.
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == app.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            app.on_exit(Box::new(exit));
                            *control_flow = ControlFlow::Exit},
                        WindowEvent::Resized(physical_size) => {
                            app.on_resize(*physical_size)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.on_resize(**new_inner_size)
                        }
                        _ => {}
                    }
                }
                //I am unsure about the ordering of this, when is this happening in the course of the program?
                Event::RedrawRequested(_) => {
                    let now = std::time::Instant::now();
                    let delta_time = now - last_render_time;
                    last_render_time = now;
                    app.on_update(delta_time);

                    app.on_draw(&mut control_flow);
                }
                _ => {}
            }//End match statement.
        });//End Run Loop.
    }//End Run function.

    fn on_draw(&mut self, control_flow: &mut winit::event_loop::ControlFlow) {
        match self.state.render(&self.camera_gpu_object)  {
            Ok(_) => {}
            // Recreate the swap_chain if lost
            Err(wgpu::SwapChainError::Lost) => self.state.resize(self.state.size),
            // The system is out of memory, we should probably quit
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }

    fn on_input(&mut self, event: &DeviceEvent, input: &winit_input_helper::WinitInputHelper){
        
        &(self.input_call.as_ref().unwrap())(event);
        
        /*match event {
            DeviceEvent::Key(
                KeyboardInput {
                    virtual_keycode: Some(key),
                    state,
                    ..
                }
            ) => self.camera.controller.process_keyboard(*key, *state),
            DeviceEvent::MouseWheel { delta, .. } => {
                self.camera.controller.process_scroll(delta);
                true
            }
            DeviceEvent::Button {
                button: 1, // Left Mouse Button
                state,
            } => {
                self.mouse_pressed = *state == ElementState::Pressed;
                true
            }
            DeviceEvent::MouseMotion { delta } => {
                if self.mouse_pressed {
                    self.camera.controller.process_mouse(delta.0, delta.1);
                }
                true
            }
            _ => false,
        }*/
    }

    fn on_exit(&self, call: CallOut){
        call();
    }

    pub fn on_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>){
        self.state.resize(physical_size);
        &(self.resize_call.as_ref().unwrap())();
        //self.camera.resize(physical_size);
    }

    pub fn on_update(&mut self, time: std::time::Duration){
        //self.camera.controller.update_camera(&mut self.camera.camera, time);
        //self.camera.update();
        //self.state.write_buffer(&self.camera_gpu_object.buffer, self.camera.uniforms)
        &(self.update_call.as_ref().unwrap())(time);
    }
}

pub trait CallBack {
     fn exit(&self);
    
     fn resize(&self);
    
     fn update(&self, dt: std::time::Duration);
    
     fn input(&self, event: &DeviceEvent);
}