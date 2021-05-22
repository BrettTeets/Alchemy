use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::{ event::*};
use futures;

use crate::graphics;

pub struct AppWindow{
    width: u32,
    height: u32,
    state: graphics::State,
    window: winit::window::Window,
    
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
        let state = block_on(graphics::State::new(&window));

        return Self{
            width,
            height,
            state,
            window,
        }
    }

    pub fn run(event_loop: EventLoop<()>, mut app: Self) -> !
    {
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
                            app.on_exit();
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
        match self.state.render()  {
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
        self.state.input(event);
    }

    fn on_exit(&self){

    }

    pub fn on_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>){
        self.state.resize(physical_size);
    }

    pub fn on_update(&mut self, time: std::time::Duration){
        self.state.update(time);
    }
}