use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use futures;

use super::*;

///Combines the graphics device state and windowing into a single object.
pub struct Graphics{
    pub state: device::State,
    pub window: winit::window::Window,
}

impl Graphics{
    pub fn new(width: u32, height: u32, title: &str, event_loop: &EventLoop<()>) -> Self{
        let window = {
            let size = LogicalSize::new(width as f64, height as f64);
            WindowBuilder::new()
                .with_title(title)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
            };

        use futures::executor::block_on;
        let state = block_on(device::State::new(&window));
        
        return Graphics{
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