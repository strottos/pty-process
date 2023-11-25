use std::os::unix::process::CommandExt as _;

use super::command::Command;
use super::pty::Pty;

impl Command {
    /// See [`std::os::unix::process::CommandExt::uid`]
    pub fn uid(&mut self, id: u32) -> &mut Self {
        self.inner.uid(id);
        self
    }

    /// See [`std::os::unix::process::CommandExt::gid`]
    pub fn gid(&mut self, id: u32) -> &mut Self {
        self.inner.gid(id);
        self
    }

    /// See [`std::os::unix::process::CommandExt::pre_exec`]
    #[allow(clippy::missing_safety_doc)]
    pub unsafe fn pre_exec<F>(&mut self, f: F) -> &mut Self
    where
        F: FnMut() -> std::io::Result<()> + Send + Sync + 'static,
    {
        self.pre_exec = Some(Box::new(f));
        self
    }

    /// See [`std::os::unix::process::CommandExt::arg0`]
    pub fn arg0<S>(&mut self, arg: S) -> &mut Self
    where
        S: AsRef<std::ffi::OsStr>,
    {
        self.inner.arg0(arg);
        self
    }

    /// Executes the command as a child process via
    /// [`std::process::Command::spawn`] on the given pty. The pty will be
    /// attached to all of `stdin`, `stdout`, and `stderr` of the child,
    /// unless those file descriptors were previously overridden through calls
    /// to [`stdin`](Self::stdin), [`stdout`](Self::stdout), or
    /// [`stderr`](Self::stderr). The newly created child process will also be
    /// made the session leader of a new session, and will have the given
    /// pty set as its controlling terminal.
    ///
    /// # Errors
    /// Returns an error if we fail to allocate new file descriptors for
    /// attaching the pty to the child process, or if we fail to spawn the
    /// child process (see the documentation for
    /// [`std::process::Command::spawn`]), or if we fail to make the child a
    /// session leader or set its controlling terminal.
    pub fn spawn(
        &mut self,
        pts: &crate::blocking::Pts,
    ) -> crate::Result<std::process::Child> {
        let (stdin, stdout, stderr) = pts.0.setup_subprocess()?;

        if !self.stdin {
            self.inner.stdin(stdin);
        }
        if !self.stdout {
            self.inner.stdout(stdout);
        }
        if !self.stderr {
            self.inner.stderr(stderr);
        }

        let mut session_leader = pts.0.session_leader();
        // Safety: setsid() is an async-signal-safe function and ioctl() is a
        // raw syscall (which is inherently async-signal-safe).
        if let Some(mut custom) = self.pre_exec.take() {
            unsafe {
                self.inner.pre_exec(move || {
                    session_leader()?;
                    custom()?;
                    Ok(())
                })
            };
        } else if !self.pre_exec_set {
            unsafe { self.inner.pre_exec(session_leader) };
        }
        self.pre_exec_set = true;

        Ok(self.inner.spawn()?)
    }
}

impl From<Pty> for std::os::fd::OwnedFd {
    fn from(pty: Pty) -> Self {
        pty.0.into()
    }
}

impl std::os::fd::AsFd for Pty {
    fn as_fd(&self) -> std::os::fd::BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl std::os::fd::AsRawFd for Pty {
    fn as_raw_fd(&self) -> std::os::fd::RawFd {
        self.0.as_raw_fd()
    }
}
