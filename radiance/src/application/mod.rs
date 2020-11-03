mod application;
pub mod utils;

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use windows::Platform;

#[cfg(target_os = "macos")]
mod mac;

#[cfg(target_os = "macos")]
pub use mac::Platform;

pub use application::{Application, ApplicationExtension, ApplicationCallbacks, DefaultApplication};
