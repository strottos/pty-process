/// Wrapper around [`std::process::Command`]
pub struct Command {
    pub(crate) inner: std::process::Command,
    pub(crate) stdin: bool,
    pub(crate) stdout: bool,
    pub(crate) stderr: bool,
    pub(crate) pre_exec_set: bool,
    pub(crate) pre_exec: Option<
        Box<dyn FnMut() -> std::io::Result<()> + Send + Sync + 'static>,
    >,
}

impl Command {
    /// See [`std::process::Command::new`]
    pub fn new<S: AsRef<std::ffi::OsStr>>(program: S) -> Self {
        Self {
            inner: std::process::Command::new(program),
            stdin: false,
            stdout: false,
            stderr: false,
            pre_exec_set: false,
            pre_exec: None,
        }
    }

    /// See [`std::process::Command::arg`]
    pub fn arg<S: AsRef<std::ffi::OsStr>>(&mut self, arg: S) -> &mut Self {
        self.inner.arg(arg);
        self
    }

    /// See [`std::process::Command::args`]
    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        self.inner.args(args);
        self
    }

    /// See [`std::process::Command::env`]
    pub fn env<K, V>(&mut self, key: K, val: V) -> &mut Self
    where
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        self.inner.env(key, val);
        self
    }

    /// See [`std::process::Command::envs`]
    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<std::ffi::OsStr>,
        V: AsRef<std::ffi::OsStr>,
    {
        self.inner.envs(vars);
        self
    }

    /// See [`std::process::Command::env_remove`]
    pub fn env_remove<K: AsRef<std::ffi::OsStr>>(
        &mut self,
        key: K,
    ) -> &mut Self {
        self.inner.env_remove(key);
        self
    }

    /// See [`std::process::Command::env_clear`]
    pub fn env_clear(&mut self) -> &mut Self {
        self.inner.env_clear();
        self
    }

    /// See [`std::process::Command::current_dir`]
    pub fn current_dir<P: AsRef<std::path::Path>>(
        &mut self,
        dir: P,
    ) -> &mut Self {
        self.inner.current_dir(dir);
        self
    }

    /// See [`std::process::Command::stdin`]
    pub fn stdin<T: Into<std::process::Stdio>>(
        &mut self,
        cfg: T,
    ) -> &mut Self {
        self.stdin = true;
        self.inner.stdin(cfg);
        self
    }

    /// See [`std::process::Command::stdout`]
    pub fn stdout<T: Into<std::process::Stdio>>(
        &mut self,
        cfg: T,
    ) -> &mut Self {
        self.stdout = true;
        self.inner.stdout(cfg);
        self
    }

    /// See [`std::process::Command::stderr`]
    pub fn stderr<T: Into<std::process::Stdio>>(
        &mut self,
        cfg: T,
    ) -> &mut Self {
        self.stderr = true;
        self.inner.stderr(cfg);
        self
    }
}
