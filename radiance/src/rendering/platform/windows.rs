// For Windows
#[cfg(target_os = "windows")]
use winapi::shared::windef::{HWND, RECT};
#[cfg(target_os = "windows")]
use winapi::um::winuser;

// For MacOS
#[cfg(target_os = "macos")]
use ash::extensions::mvk::MacOSSurface;
#[cfg(target_os = "macos")]
use cocoa::appkit::{NSView, NSWindow};
#[cfg(target_os = "macos")]
use cocoa::base::id as cocoa_id;
#[cfg(target_os = "macos")]
use metal::CoreAnimationLayer;
#[cfg(target_os = "macos")]
use objc::runtime::YES;
use winit::platform::macos::WindowExtMacOS;

#[cfg(target_os = "macos")]
use winit;

#[cfg(target_os = "windows")]
#[derive(Copy, Clone)]
pub struct Window {
    pub hwnd: HWND,
}

#[cfg(target_os = "windows")]
impl Window {
    pub fn size(&self) -> (u32, u32) {
        unsafe {
            let mut rc: RECT = std::mem::zeroed();
            winuser::GetWindowRect(self.hwnd, &mut rc);
            ((rc.right - rc.left) as u32, (rc.bottom - rc.top) as u32)
        }
    }
}

#[cfg(target_os = "macos")]
// #[derive(Copy, Clone)]
pub struct Window<'a> {
    pub window: &'a winit::window::Window,
}
