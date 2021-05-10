use log::error;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
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
        
        use futures::executor::block_on;
        let state = block_on(graphics::State::new(&window));

        return Self{
            width,
            height,
            state,
            window,
        }
    }

    pub fn run(event_loop: EventLoop<()>, mut domain: Self) -> !
    {
        
        //let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();

        event_loop.run(move |event, _, control_flow| {
            // Draw the current frame
            if let Event::RedrawRequested(_) = event {
                domain.draw();
                if false
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
    
            // Handle input events
            if input.update(&event) {
                // Close events
                if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
    
                // Resize the window
                if let Some(size) = input.window_resized() {
                    //domain.pixels.resize_surface(size.width, size.height);
                }
    
                // Update internal state and request a redraw
                //world.update();
                domain.window.request_redraw();
            }
        });
    }

    fn draw(&mut self) {
        
    }
}