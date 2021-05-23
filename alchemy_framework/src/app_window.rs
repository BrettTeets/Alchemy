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
            state,
            window,
        }
    }


    pub fn init_graphics_device(&mut self, bind_group_layout: &wgpu::BindGroupLayout){
        self.state.init_pipeline(&bind_group_layout);
    }
 
    pub fn on_draw(&mut self, camera_gpu_object: &crate::camera::GPUObject<crate::camera::Uniforms>, control_flow: &mut winit::event_loop::ControlFlow) {
        match self.state.render(&camera_gpu_object)  {
            Ok(_) => {}
            // Recreate the swap_chain if lost
            Err(wgpu::SwapChainError::Lost) => self.state.resize(self.state.size),
            // The system is out of memory, we should probably quit
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }

    pub fn on_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>){
        self.state.resize(physical_size);
    }
}