use alchemy_framework as alchemy;
use winit::{
    event::*,
    event_loop::{ControlFlow},
};


fn main() {
    let config = alchemy::graphics::WindowConfig::new(800.0, 800.0, "Hello Wolrd".to_string());
    <GameEngine as alchemy::graphics::App>::run(config).expect("something went wrong");
}

pub struct GameEngine{
    camera: alchemy_framework::camera::CameraObject,
    pub camera_gpu_object: alchemy_framework::camera::GPUObject<alchemy_framework::camera::Uniforms>,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,
    mouse_pressed: bool,
}

impl alchemy::graphics::App for GameEngine{
    
    fn new(gpu: &alchemy::gpu::State) -> Self { 
        let uniform_bind_group_layout = gpu.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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

        let mut camera = alchemy_framework::camera::CameraObject::new(&gpu.sc_desc);
        camera.update();
        let camera_gpu_object = alchemy_framework::camera::GPUObject::new(&gpu.device, &uniform_bind_group_layout, camera.uniforms);

        Self{
            camera,
            camera_gpu_object,
            uniform_bind_group_layout,
            mouse_pressed: false,
        }
    }

    fn on_load(&self, app: &mut alchemy::graphics::AppWindow) { 
        let vert = wgpu::include_spirv!("shader.vert.spv");
        let frag = wgpu::include_spirv!("shader.frag.spv");

        let render_pipeline_layout =
        app.gpu.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&self.uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        app.gpu.init_pipeline(render_pipeline_layout, vert, frag);
    }


    fn on_update(&mut self, app: &mut alchemy::graphics::AppWindow, delta: std::time::Duration) { 
        self.camera.controller.update_camera(&mut self.camera.camera, delta);
        self.camera.update();
        app.gpu.write_buffer(&self.camera_gpu_object.buffer, self.camera.uniforms)
    }

    fn on_draw(&self, app: &mut alchemy::graphics::AppWindow, control_flow: &mut winit::event_loop::ControlFlow) { 
        match app.gpu.render(&self.camera_gpu_object)  {
            Ok(_) => {}
            // Recreate the swap_chain if lost
            Err(wgpu::SwapChainError::Lost) => app.gpu.resize(app.gpu.size),
            // The system is out of memory, we should probably quit
            Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }

    fn on_input(&mut self, event: &winit::event::DeviceEvent) {      
        let _ = match event {
            DeviceEvent::Key(KeyboardInput {
                virtual_keycode: Some(key),
                state,
                ..
            }) => self.camera.controller.process_keyboard(*key, *state),
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
        };
    }

    fn on_resize(&mut self, physical_size: winit::dpi::PhysicalSize<u32>) 
    { 
        self.camera.resize(physical_size);
    }

    fn on_exit(&self) { 
        //todo!() 
    }
}









