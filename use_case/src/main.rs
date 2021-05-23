use alchemy_framework as alchemy;
use alchemy::app_window::AppWindow;
use winit::event_loop::EventLoop;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 800;

fn main() {
    let event_loop = EventLoop::new();
    let mut app_window: AppWindow = AppWindow::new(HEIGHT, WIDTH, &event_loop);
    
    let example: Example = Example::new(&app_window);
    app_window.init_graphics_device(&example.uniform_bind_group_layout);
    app_window.init_graphics_device(&example.uniform_bind_group_layout);
    
    AppWindow::run(event_loop, app_window, example);
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

    
    
}

impl alchemy::app_window::CallBack for Example{
     fn exit(&self){
        println!("If this works we are cooking with fire.");
    }
    
     fn resize(&self){
        println!("If this works we are cooking with fire.");
    }
    
     fn update(&self, dt: std::time::Duration){
        println!("If this works we are cooking with fire.");
    }
    
    
     fn input(&self, event: &DeviceEvent){
        println!("If this works we are cooking with fire.");
    }
}




