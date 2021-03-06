use std::{fs, io as stdio, path::Path};

use prelude::*;

pub struct Stdout {
    _private: (),
}

pub struct IO {
    _private: (),
}

impl Capability for IO {
    unsafe fn construct() -> Self {
        IO { _private: () }
    }
}

impl IO {
    pub fn write_file(
        &self,
        path: impl AsRef<Path>,
        contents: impl AsRef<[u8]>,
    ) -> stdio::Result<()> {
        fs::write(path, contents)
    }

    pub fn read_file(&self, path: impl AsRef<Path>) -> stdio::Result<Vec<u8>> {
        fs::read(path)
    }

    pub fn stdin(&self) -> stdio::Stdin {
        stdio::stdin()
    }

    pub fn stdout(&self, str: &str) {
        print!("{str}")
    }
}

#[derive(Debug)]
pub struct AuthErr;
impl std::fmt::Display for AuthErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The authorizer did not permit the operation")
    }
}
impl std::error::Error for AuthErr {}

type AuthResult<T> = Result<T, AuthErr>;

pub struct TempIO<F>
where
    F: FnMut(Operation) -> bool,
{
    stdio: IO,
    authorizer: F,
}

#[non_exhaustive]
pub enum Operation<'call> {
    WriteFile(&'call dyn AsRef<Path>, &'call dyn AsRef<[u8]>),
    ReadFile(&'call dyn AsRef<Path>),
    StdIn,
    StdOut(&'call str),
}

impl<F> TempIO<F>
where
    F: FnMut(Operation) -> bool,
{
    pub fn new(stdio: IO, authorizer: F) -> Self {
        Self { stdio, authorizer }
    }

    pub fn write_file(
        &mut self,
        path: impl AsRef<Path> + Clone,
        contents: impl AsRef<[u8]> + Clone,
    ) -> AuthResult<stdio::Result<()>> {
        if (self.authorizer)(Operation::WriteFile(&path, &contents)) {
            return Ok(self.stdio.write_file(path, contents));
        }
        Err(AuthErr)
    }

    pub fn read_file(&mut self, path: impl AsRef<Path>) -> AuthResult<stdio::Result<Vec<u8>>> {
        if (self.authorizer)(Operation::ReadFile(&path)) {
            return Ok(self.stdio.read_file(path));
        }
        Err(AuthErr)
    }

    pub fn stdin(&mut self) -> AuthResult<stdio::Stdin> {
        if (self.authorizer)(Operation::StdIn) {
            return Ok(self.stdio.stdin());
        }
        Err(AuthErr)
    }

    pub fn stdout(&mut self, str: &str) -> AuthResult<()> {
        if (self.authorizer)(Operation::StdOut(str)) {
            #[allow(clippy::unit_arg)]
            return Ok(self.stdio.stdout(str));
        }
        Err(AuthErr)
    }
}
