//! Functions to write and read the secret to/from a file.
use std::{
    fs::{File, OpenOptions},
    io::prelude::*,
    path::Path,
};

use rand::{RngExt, distr::Alphanumeric};

use crate::error::{Error, IoError};

/// Read the shared secret from a file.
pub fn read_shared_secret(path: &Path) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path).io_path(
        path,
        "opening secret file. Did you start the daemon at least once?",
    )?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .io_path(path, "reading secret file")?;

    Ok(buffer)
}

/// Generate a random secret and write it to a file.
pub fn init_shared_secret(path: &Path) -> Result<(), Error> {
    if path.exists() {
        return Ok(());
    }

    const PASSWORD_LEN: usize = 512;
    let mut rng = rand::rng();

    let secret: String = std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(PASSWORD_LEN)
        .collect();

    let mut options = OpenOptions::new();
    options.write(true).create_new(true);

    #[cfg(not(target_os = "windows"))]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o640);
    }

    let mut file = options.open(path).io_path(path, "creating shared secret")?;
    file.write_all(&secret.into_bytes())
        .io_path(path, "writing shared secret")?;

    Ok(())
}
