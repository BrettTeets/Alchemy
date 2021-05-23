use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::{ event::*};
use crate::gpu;

pub trait App {
    fn new(gpu: &gpu::State) -> Self;

    fn run(window_config: WindowConfig) -> Result<(), ()>
    where
        Self: 'static + Sized,
    {
        return <Default as Go<Self>>::run(window_config);
    }

    fn on_load(&self, app: &mut AppWindow,);

    fn on_update(&mut self, app: &mut AppWindow, delta: std::time::Duration);

    fn on_draw(&self, app: &mut AppWindow, control_flow: &mut winit::event_loop::ControlFlow);

    fn on_input(&mut self, inputs: &winit::event::DeviceEvent);

    fn on_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>);

    fn on_exit(&self);
    
}

pub struct Default {}

impl<A: App> Go<A> for Default
where
    A: 'static,
{
    fn new(
        game: &mut A,
        window: &AppWindow,
    ) -> Self{
        return Self {};
    }
}

pub trait Go<A: App>{
    fn new(
        app: &mut A,
        window: &AppWindow,
    ) -> Self;

    fn run(window_config: WindowConfig) -> Result<(), ()>
    where
    Self: 'static + Sized,
    A: 'static,
    {
        let event_loop = EventLoop::new();
        let mut window = window_config.make_window(&event_loop);
        let mut app = App::new(&window.gpu);
        let mut game_loop = Self::new(&mut app, &mut window); //If you take this out app requires type annotation, since I might 
        //need to add in some underlying functionality in the future that will go here no point in refactoring to fix that.

        app.on_load(&mut window);

        let mut input = WinitInputHelper::new();
        let mut last_render_time = std::time::Instant::now();
    
        event_loop.run(move |event, _, mut control_flow| {
            input.update(&event);
            if input.key_pressed(VirtualKeyCode::Escape) {*control_flow = ControlFlow::Exit;}
            
            match event {
                //Event main events are cleared with request a redraw?
                Event::MainEventsCleared => window.window.request_redraw(),
                Event::DeviceEvent { event, ..} => {
                    app.on_input(&event);
                }
                //Handle window specific events and other things winit picks up I guess.
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == window.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            app.on_exit();
                            *control_flow = ControlFlow::Exit},
                        WindowEvent::Resized(physical_size) => {
                            app.on_resize(*physical_size);
                            window.gpu.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.on_resize(**new_inner_size);
                            window.gpu.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
                //I am unsure about the ordering of this, when is this happening in the course of the program?
                Event::RedrawRequested(_) => {
                    let now = std::time::Instant::now();
                    let delta_time = now - last_render_time;
                    last_render_time = now;
                    app.on_update(&mut window, delta_time);
                    app.on_draw(&mut window, &mut control_flow);
                }
                _ => {}
            }//End match statement.
        });//End Run Loop.
    }//end Run Function
}

pub struct AppWindow{
    pub gpu: gpu::State,
    pub window: winit::window::Window,
}

pub struct WindowConfig{
    pub width: f64,
    pub height: f64,
    pub title: String,
}

impl WindowConfig{
    pub fn new(width: f64, height: f64, title: String) -> Self{
        Self{
            width,
            height,
            title,
        }
    }

    pub fn make_window(&self, event_loop: &EventLoop<()>) -> AppWindow {
        let window = {
            use winit::window::WindowBuilder;
            use winit::dpi::LogicalSize;
            let size = LogicalSize::new(self.width, self.height);
            WindowBuilder::new()
                .with_title(self.title.as_str())
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };

        use futures::executor::block_on;
        let gpu = block_on(gpu::State::new(&window));
        
        return AppWindow{
            gpu,
            window,
        }
    }
}