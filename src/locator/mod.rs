#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use linux::find_executable_in_path;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::find_executable_in_path;

#[cfg(target_os = "windows")]
mod windows;
mod tests;

#[cfg(target_os = "windows")]
pub use windows::find_executable_in_path;