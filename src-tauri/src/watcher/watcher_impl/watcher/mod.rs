#[cfg(not(windows))]
pub mod not_windows;
#[cfg(windows)]
pub mod windows;

#[cfg(windows)]
pub use windows::WMIWatcher as WatcherImpl;

#[cfg(not(windows))]
pub use not_windows::NotWindowsWatcher as WatcherImpl;
