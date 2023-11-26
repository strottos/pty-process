use super::command::Command;

impl Command {
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
        todo!()
    }
}
