#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

#[cfg(unix)]
pub type Pty = unix::UnixPty;
#[cfg(windows)]
pub type Pty = windows::WindowsPty;

#[cfg(unix)]
pub type Pts = unix::UnixPts;
#[cfg(windows)]
pub type Pts = windows::WindowsPts;
