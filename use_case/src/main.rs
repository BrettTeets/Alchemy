use alchemy_framework as alchemy;
use alchemy::app_window::AppWindow;
use winit::event_loop::EventLoop;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 800;

fn main() {
    let event_loop = EventLoop::new();
    let domain = AppWindow::new(HEIGHT, WIDTH, &event_loop);
    AppWindow::run(event_loop, domain);
}
