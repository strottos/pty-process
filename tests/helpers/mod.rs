#![allow(dead_code)]

use std::io::BufRead as _;

pub struct Output<'a> {
    pty: std::io::BufReader<&'a pty_process::blocking::Pty>,
}

impl<'a> Output<'a> {
    fn new(pty: &'a pty_process::blocking::Pty) -> Self {
        Self {
            pty: std::io::BufReader::new(pty),
        }
    }

    #[cfg(unix)]
    fn unix_read_until(
        &mut self,
        buf: &mut Vec<u8>,
    ) -> std::io::Result<usize> {
        nix::unistd::alarm::set(5);
        let size = self.pty.read_until(b'\n', buf).unwrap();
        nix::unistd::alarm::cancel();

        Ok(size)
    }

    #[cfg(windows)]
    fn windows_read_until(
        &mut self,
        buf: &mut Vec<u8>,
    ) -> std::io::Result<usize> {
        // TODO: Timeout
        let size = self.pty.read_until(b'\n', buf).unwrap();

        Ok(size)
    }
}

impl<'a> Iterator for Output<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = vec![];
        #[cfg(unix)]
        self.unix_read_until(&mut buf).unwrap();
        #[cfg(windows)]
        self.windows_read_until(&mut buf).unwrap();
        Some(std::string::String::from_utf8(buf).unwrap())
    }
}

pub fn output(pty: &pty_process::blocking::Pty) -> Output<'_> {
    Output::new(pty)
}

#[cfg(feature = "async")]
pub fn output_async<'a>(
    pty: impl tokio::io::AsyncRead + std::marker::Unpin + 'a,
) -> std::pin::Pin<Box<dyn futures::stream::Stream<Item = String> + 'a>> {
    use futures::FutureExt as _;
    use tokio::io::AsyncBufReadExt as _;

    let pty = tokio::io::BufReader::new(pty);
    Box::pin(futures::stream::unfold(pty, |mut pty| async move {
        Some((
            tokio::time::timeout(std::time::Duration::from_secs(5), async {
                let mut buf = vec![];
                pty.read_until(b'\n', &mut buf).await.unwrap();
                std::string::String::from_utf8(buf).unwrap()
            })
            .map(|x| x.unwrap())
            .await,
            pty,
        ))
    }))
}
