use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE},
    System::{
        Console::{CreatePseudoConsole, ResizePseudoConsole, COORD, HPCON},
        Pipes::CreatePipe,
    },
};

#[derive(Debug)]
pub struct WindowsPty {
    pseudo_terminal: HPCON,
    stdin: HANDLE,
    stdout: HANDLE,
}

impl WindowsPty {
    pub fn open() -> crate::Result<Self> {
        let handle;
        let mut stdin = INVALID_HANDLE_VALUE;
        let mut stdout = INVALID_HANDLE_VALUE;
        let mut h_pipe_pty_in = INVALID_HANDLE_VALUE;
        let mut h_pipe_pty_out = INVALID_HANDLE_VALUE;

        unsafe {
            CreatePipe(&mut h_pipe_pty_in, &mut stdin, None, 0)?;
            CreatePipe(&mut stdout, &mut h_pipe_pty_out, None, 0)?;
        }

        let console_size = COORD { X: 80, Y: 24 };

        unsafe {
            handle = CreatePseudoConsole(
                console_size,
                h_pipe_pty_in,
                h_pipe_pty_out,
                0,
            )?;

            CloseHandle(h_pipe_pty_in)?;
            CloseHandle(h_pipe_pty_out)?;
        }

        Ok(WindowsPty {
            pseudo_terminal: handle,
            stdin,
            stdout,
        })
    }

    pub fn set_term_size(&self, size: crate::Size) -> crate::Result<()> {
        let console_size = COORD {
            X: i16::try_from(size.cols()).map_err(|x| x)?,
            Y: i16::try_from(size.rows())?,
        };
        unsafe {
            ResizePseudoConsole(self.pseudo_terminal, console_size)?;
        }

        Ok(())
    }

    pub fn pts(&self) -> crate::Result<WindowsPts> {
        Ok(WindowsPts())
    }
}

impl std::io::Read for WindowsPty {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl std::io::Write for WindowsPty {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

impl std::io::Read for &WindowsPty {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        todo!()
    }
}

impl std::io::Write for &WindowsPty {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}

#[derive(Debug)]
pub struct WindowsPts();
