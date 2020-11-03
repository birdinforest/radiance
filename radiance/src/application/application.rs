use super::Platform;
use crate::constants;
use crate::radiance;
use crate::radiance::CoreRadianceEngine;
use crate::rendering;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::time::Instant;

pub trait ApplicationCallbacks {
    fn on_initialized<T: ApplicationCallbacks + ApplicationExtension<T>>(&mut self, _: &mut Application<T>);
    fn on_updated<T: ApplicationCallbacks + ApplicationExtension<T>>(&mut self, _: &mut Application<T>, _: f32);
}

pub trait ApplicationExtension<TImpl: ApplicationExtension<TImpl>> {
    define_ext_fn!(on_initialized, Application, TImpl);
    define_ext_fn!(on_updating, Application, TImpl, _delta_sec: f32);
}

mod private {
    pub struct EmptyCallbacks {}
    impl super::ApplicationExtension<EmptyCallbacks> for EmptyCallbacks {}
}
pub type DefaultApplication = Application<private::EmptyCallbacks>;

pub struct Application<TExtension: ApplicationExtension<TExtension>> {
    radiance_engine: CoreRadianceEngine,
    platform: Platform,
    extension: Rc<RefCell<TExtension>>,
}

impl<TExtension: ApplicationExtension<TExtension>> Application<TExtension> {
    #[cfg(target_os = "windows")]
    pub fn new(extension: TExtension) -> Self {
        set_panic_hook();
        let platform = Platform::new();

        let window = rendering::Window {
            hwnd: platform.hwnd(),
        };
        Self {
            radiance_engine: radiance::create_radiance_engine(&window)
                .expect(constants::STR_FAILED_CREATE_RENDERING_ENGINE),
            platform,
            extension: Rc::new(RefCell::new(extension)),
        }
    }

    #[cfg(target_os = "macos")]
    pub unsafe fn new(extension: TExtension) -> Self {
        set_panic_hook();
        let platform = Platform::new();
        let window = rendering::Window {
            window: &platform.window,
        };
        Self {
            radiance_engine: radiance::create_radiance_engine(&window)
                .expect(constants::STR_FAILED_CREATE_RENDERING_ENGINE),
            platform,
            extension: Rc::new(RefCell::new(extension)),
        }
    }

    pub fn engine_mut(&mut self) -> &mut CoreRadianceEngine {
        &mut self.radiance_engine
    }

    pub fn callbacks_mut(&self) -> RefMut<TExtension> {
        self.extension.borrow_mut()
    }

    #[cfg(target_os = "windows")]
    pub fn initialize(&mut self) {
        self.platform.initialize();
        ext_call!(self, on_initialized);
    }

    #[cfg(target_os = "macos")]
    pub fn initialize(&mut self) {
        self.platform.initialize();
        ext_call!(self, on_initialized);
    }

    #[cfg(target_os = "windows")]
    pub fn set_title(&mut self, title: &str) {
        self.platform.set_title(title);
    }

    pub fn run(&mut self) {
        let mut frame_start_time = Instant::now();
        let mut elapsed = 0.;
        loop {
            #[cfg(target_os = "windows")]
            if !self.platform.process_message() {
                break;
            }

            let frame_end_time = Instant::now();
            elapsed = frame_end_time
                .duration_since(frame_start_time)
                .as_secs_f32();

            /*if elapsed < 1./120. {
                continue;
            }*/

            frame_start_time = frame_end_time;
            ext_call!(self, on_updating, elapsed);

            self.radiance_engine.update(elapsed);
        }

        self.radiance_engine.unload_scene();
    }
}

fn set_panic_hook() {
    std::panic::set_hook(Box::new(|panic_info| {
        let backtrace = backtrace::Backtrace::new();
        let msg = format!("Radiance {}\n{:?}", panic_info, backtrace);
        Platform::show_error_dialog(crate::constants::STR_SORRY_DIALOG_TITLE, &msg);
    }));
}
