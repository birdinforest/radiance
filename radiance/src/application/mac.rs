use winit::window::{Window, WindowBuilder};  // Used to create window.
use winit::event_loop::{EventLoop};  // Used to create window.

pub fn window_create(
    title: &'static str,            // What is `&'static str` ??
    width: u32,
    height: u32,
    events_loop: &EventLoop<()>,
) -> Window {
    WindowBuilder::new()
        .with_title(title)                          // Set title
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))    // Set size
        .build(events_loop)                         // Set events loop
        .expect("Err - Window creation failed: ")
}

pub struct Platform {
    pub window: winit::window::Window,
}

pub const WINDOW_TITLE: &'static str = "Radiance";

impl Platform {
    pub fn show_error_dialog(title: &str, msg: &str) {
        unsafe {
            println!(
                "\nPlatform::mac error. title: {}, msg: {}",
                title, msg
            );
        }
    }

    pub fn new() -> Self {
        // Self::set_dpi_awareness();
        let ev = EventLoop::new();
        let window = window_create( WINDOW_TITLE, 800, 600, &ev);
        // if window.is_null() {
        //     Platform::show_error_dialog("Window Create", "Window create failed.");
        // }

        Self { window }
    }
}