use crate::env::{self, var};
use anyhow::{ensure, Context, Result};
use std::{
    env::{join_paths, split_paths},
    ffi::{OsStr, OsString},
    path::Path,
    process::{Command as StdCommand, Output, Stdio},
};

pub struct Command {
    command: StdCommand,
}

impl Command {
    pub fn new<S: AsRef<OsStr>>(program: S) -> Self {
        Self {
            command: StdCommand::new(program),
        }
    }

    pub fn args<I, S>(&mut self, args: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.command.args(args);
        self
    }

    pub fn envs<I, K, V>(&mut self, vars: I) -> &mut Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<OsStr>,
        V: AsRef<OsStr>,
    {
        self.command.envs(vars);
        self
    }

    pub fn env_remove<K: AsRef<OsStr>>(&mut self, key: K) -> &mut Self {
        self.command.env_remove(key);
        self
    }

    pub fn current_dir<P: AsRef<Path>>(&mut self, dir: P) -> &mut Self {
        self.command.current_dir(dir);
        self
    }

    pub fn stdout<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.command.stdout(cfg);
        self
    }

    pub fn stderr<T: Into<Stdio>>(&mut self, cfg: T) -> &mut Self {
        self.command.stderr(cfg);
        self
    }

    pub fn output(&mut self) -> Result<Output> {
        log::debug!("{:?}", self.command.get_envs().collect::<Vec<_>>());
        log::debug!("{:?}", self.command.get_current_dir());
        log::debug!("{:?}", self.command);

        let output = self
            .command
            .output()
            .with_context(|| format!("Could not get output of `{:?}`", self.command))?;

        ensure!(
            output.status.success(),
            "command failed: {:?}\nstdout: {:?}\nstderr: {:?}",
            self.command,
            std::str::from_utf8(&output.stdout).unwrap_or_default(),
            std::str::from_utf8(&output.stderr).unwrap_or_default()
        );

        Ok(output)
    }

    // smoelius: Why not get the status by calling `self.output()`? Because we don't want stdout and
    // stderr to be captured.
    pub fn success(&mut self) -> Result<()> {
        log::debug!("{:?}", self.command.get_envs().collect::<Vec<_>>());
        log::debug!("{:?}", self.command.get_current_dir());
        log::debug!("{:?}", self.command);

        let status = self
            .command
            .status()
            .with_context(|| format!("Could not get status of `{:?}`", self.command))?;

        ensure!(status.success(), "command failed: {:?}", self.command);

        Ok(())
    }
}

pub fn driver(toolchain: &str, driver: &Path) -> Result<Command> {
    let path = var(env::PATH)?;

    let path = {
        if cfg!(target_os = "windows") {
            // MinerSebas: To succesfully determine the dylint driver Version on Windows,
            // it is neccesary to add some Libraries to the Path.
            let rustup_home = var(env::RUSTUP_HOME)?;

            join_paths(
                std::iter::once(
                    Path::new(&rustup_home)
                        .join("toolchains")
                        .join(toolchain)
                        .join("bin"),
                )
                .chain(split_paths(&path)),
            )?
        } else {
            OsString::from(path)
        }
    };

    let mut command = Command::new(driver);
    command.envs(vec![(env::PATH, path)]);
    Ok(command)
}
