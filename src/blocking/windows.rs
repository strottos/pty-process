use windows::{
    core::{PCWSTR, PWSTR},
    Win32::System::Threading::{
        CreateProcessW, CREATE_UNICODE_ENVIRONMENT,
        EXTENDED_STARTUPINFO_PRESENT, PROCESS_INFORMATION, STARTUPINFOEXW,
    },
};

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
        let mut start_info = STARTUPINFOEXW::default();
        start_info.StartupInfo.cb =
            std::mem::size_of::<STARTUPINFOEXW>() as u32;

        let (mut exe, mut command) = self.get_program()?;

        let working_dir = std::env::current_dir()
            .expect("failed to get cwd")
            .into_os_string()
            .into_string()
            .unwrap();

        let mut start_info = STARTUPINFOEXW::default();
        start_info.StartupInfo.cb =
            std::mem::size_of::<STARTUPINFOEXW>() as u32;

        let mut p_info = PROCESS_INFORMATION::default();

        unsafe {
            CreateProcessW(
                PCWSTR(
                    exe.encode_utf16()
                        .chain(::std::iter::once(0))
                        .collect::<Vec<u16>>()
                        .as_mut_ptr(),
                ),
                PWSTR(
                    command
                        .encode_utf16()
                        .chain(::std::iter::once(0))
                        .collect::<Vec<u16>>()
                        .as_mut_ptr(),
                ),
                None,
                None,
                false,
                EXTENDED_STARTUPINFO_PRESENT | CREATE_UNICODE_ENVIRONMENT,
                cmd.environment_block().as_mut_slice().as_mut_ptr() as *mut _,
                &HSTRING::from(working_dir),
                &mut startup_info.StartupInfo,
                &mut p_info,
            )?;
        }

        Ok(())
    }

    fn get_program(&self) -> crate::Result<(String, String)> {
        let exe = self
            .exe
            .as_ref()
            .ok_or_else(|| crate::Error::NoExeSpecified)?;

        let exe = exe.to_str().ok_or_else(|| {
            crate::Error::InvalidExeSpecified(exe.to_string_lossy().into())
        })?;

        Ok(exe.to_string())
    }
}
