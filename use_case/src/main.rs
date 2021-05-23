use alchemy_framework as alchemy;
use alchemy::app_window::AppWindow;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit_input_helper::WinitInputHelper;
use winit::{ event::*};

const WIDTH: u32 = 320;
const HEIGHT: u32 = 800;

fn main() {
    let event_loop = EventLoop::new();
    let mut app_window: AppWindow = AppWindow::new(HEIGHT, WIDTH, &event_loop);
    let mut example: Example = Example::new(&app_window);
    app_window.init_graphics_device(&example.uniform_bind_group_layout);
    

    let mut input = WinitInputHelper::new();
    let mut last_render_time = std::time::Instant::now();

    event_loop.run(move |event, _, mut control_flow| {
        input.update(&event);
        if input.key_pressed(VirtualKeyCode::Escape) {*control_flow = ControlFlow::Exit;}
        
        match event {
            //Event main events are cleared with request a redraw?
            Event::MainEventsCleared => app_window.window.request_redraw(),
            //I am not handling events or device Id here, 
            Event::DeviceEvent { ref event, ..} => {
                //doubling up while we sort this code out.
                example.input(event, &input);
            }
            //Handle window specific events and other things winit picks up I guess.
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == app_window.window.id() => {
                match event {
                    WindowEvent::CloseRequested => {
                        example.exit();
                        *control_flow = ControlFlow::Exit},
                    WindowEvent::Resized(physical_size) => {
                        app_window.on_resize(*physical_size);
                        example.resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        app_window.on_resize(**new_inner_size)
                    }
                    _ => {}
                }
            }
            //I am unsure about the ordering of this, when is this happening in the course of the program?
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let delta_time = now - last_render_time;
                last_render_time = now;
                example.update(&mut app_window, delta_time);

                app_window.on_draw(&example.camera_gpu_object, &mut control_flow);
            }
            _ => {}
        }//End match statement.
    });//End Run Loop.
}

struct Example{
    camera: alchemy_framework::camera::CameraObject,
    camera_gpu_object: alchemy_framework::camera::GPUObject<alchemy_framework::camera::Uniforms>,
    uniform_bind_group_layout: wgpu::BindGroupLayout,
    mouse_pressed: bool,
}

use winit::{ event::*};
impl Example{
    fn new(app: &AppWindow) -> Self{
        let uniform_bind_group_layout = app.state.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let mut camera = alchemy_framework::camera::CameraObject::new(&app.state.sc_desc);
        camera.update();
        let camera_gpu_object = alchemy_framework::camera::GPUObject::new(&app.state.device, &uniform_bind_group_layout, camera.uniforms);

        Self{
            camera,
            camera_gpu_object,
            uniform_bind_group_layout,
            mouse_pressed: false,
        }
    }

    fn exit(&self){
        
    }
    
     fn resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>){
        self.camera.resize(physical_size);
    }
    
    fn update(&mut self, app: &mut AppWindow, dt: std::time::Duration){
        self.camera.controller.update_camera(&mut self.camera.camera, dt);
        self.camera.update();
        app.state.write_buffer(&self.camera_gpu_object.buffer, self.camera.uniforms)
    }
    
    
     fn input(&mut self, event: &DeviceEvent, helper: &winit_input_helper::WinitInputHelper) -> bool{
        match event {
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
        }
    }
}





