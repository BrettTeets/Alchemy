use log::error;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::{
    event::*,
    
};
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
        
        //let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        //For Timing purposes.
        let mut last_render_time = std::time::Instant::now();

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            match event {
                //Event main events are cleared with request a redraw?
                Event::MainEventsCleared => app.window.request_redraw(),
                //Is this the graphics device or another device?
                Event::DeviceEvent {
                    ref event,
                    .. // We're not using device_id currently
                } => {
                    app.on_input(event);
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
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => {
                                *control_flow = ControlFlow::Exit;
                            }
                            _ => {}
                        },
                        WindowEvent::Resized(physical_size) => {
                            app.on_resize(*physical_size)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.on_resize(**new_inner_size)
                        }
                        _ => {}
                    }
                }
                //now handle those redraw events.
                Event::RedrawRequested(_) => {
                    let now = std::time::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;
                    app.on_update(dt);
                    app.on_draw();
                }
                _ => {}
            } 
            
        });
    }

    fn on_draw(&mut self) {
        match self.state.render() {
            Ok(_) => {}
            // Recreate the swap_chain if lost
            Err(wgpu::SwapChainError::Lost) => self.state.resize(self.state.size),
            // The system is out of memory, we should probably quit
            //Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }

    fn on_input(&self, event: &winit::event::DeviceEvent){

    }

    fn on_exit(&self){

    }

    pub fn on_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>){
        self.state.resize(physical_size);
    }

    pub fn on_update(&mut self, time: std::time::Duration){
        
    }
}