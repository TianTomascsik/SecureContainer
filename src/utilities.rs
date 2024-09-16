//! # Utilities
//! This module provides different utility functions that are used by the rest of the project.
//!

use crate::error_handling;
use error_handling::{Result, SecureContainerErr};

extern crate libuta_rs;
use libuta_rs::libuta_derive_key;

use crate::file_io_operations;
use file_io_operations::auto_open_read;

use crate::cryptsetup_wrapper;
use cryptsetup_wrapper::{close_container, open_container};

use std::process::Command;

use crate::error_handling::check_input;
use base64::engine::general_purpose;
use base64::{alphabet, engine, Engine as _};

/// Get the password for a container.
/// # Arguments
/// * `id` - The id of the container.
/// # Returns
/// * `Result<String>` -
/// Returns a `String` containing the password if successful otherwise an error is returned.
/// # Errors
/// * `LibutaDeriveKeyError` - An error occurred while deriving the key.
/// # Example
/// ```
/// let id = "test";
/// let result = get_password(id);
/// println!("{:?}", result.unwrap());
/// ```
///
pub fn get_password(id: &str) -> Result<String> {
    let key = match libuta_derive_key(id) {
        Ok(key) => key,
        Err(err) => return Err(SecureContainerErr::LibutaDeriveKeyError(err.to_string())),
    };
    let password = convert_to_base64(key);
    Ok(password)
}

/// Function that is called by the daemon to automatically open all containers in autoOpen file.
/// # Arguments
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if all containers were opened successfully, otherwise an error is returned.
/// # Errors
/// * `FileReadError` - An error occurred while reading a file.
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// * `ContainerOpen` - The container is already open.
/// * `LibutaDeriveKeyError` - An error occurred while deriving the key.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// * `IntegrityError` - The integrity check failed.
/// * `LsblkError` - A contaienr with the given name does not exist.
/// * `MkfsError` - An error occurred creation the file system.
/// * `MountError` - An error occurred while trying to mount the container.
/// # Example
/// ```
/// let result = auto_open();
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn auto_open() -> Result<()> {
    let containers = auto_open_read();
    if containers.is_err() {
        return Err(SecureContainerErr::FileReadError(
            "Error reading auto open file".to_string(),
        ));
    }
    for container in containers.unwrap() {
        match check_input(
            None,
            Some(&container[0]),
            Some(&container[1]),
            Some(&container[2]),
            Some(&container[3]),
        ) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
        match open_container(&container[0], &container[1], &container[2], &container[3]) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
    }
    Ok(())
}

/// Function that is called by the daemon to close all containers in autoOpen file.
/// # Arguments
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if all containers were closed successfully, otherwise an error is returned.
/// # Errors
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `UmountError` - An error occurred while the container was unmounted.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// # Example
/// ```
/// let result = auto_close();
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn auto_close() -> Result<()> {
    let containers = auto_open_read();
    if containers.is_err() {
        return Err(SecureContainerErr::FileReadError(
            "Error reading auto open file".to_string(),
        ));
    }
    let containers = containers.unwrap();
    let mut is_closed = vec![false; containers.len()];

    while is_closed.contains(&false) {
        for container in &containers {
            if !is_closed[containers.iter().position(|x| x == container).unwrap()] {
                let returncode = close_container(&container[0], &container[2]);
                if returncode.is_ok() {
                    is_closed[containers.iter().position(|x| x == container).unwrap()] = true;
                }
            }
        }
    }
    Ok(())
}

/// Converts a byte stream to a base64 string.
/// # Arguments
/// * `binary` - The byte stream to convert.
/// # Returns
/// * `String` -
/// Returns a `String` containing the base64 encoded byte stream.
/// # Errors
/// # Example
/// ```
/// let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
/// let output = convert_to_base64(input);
/// assert_eq!(output, "AAECAwQFBgcICQ");
/// ```
///
pub fn convert_to_base64(binary: Vec<u8>) -> String {
    let alphabet =
        alphabet::Alphabet::new("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/")
            .unwrap();
    let engine: engine::GeneralPurpose =
        engine::GeneralPurpose::new(&alphabet, general_purpose::NO_PAD);
    let password = engine.encode(binary);
    password
}

/// Converts MB in bytes.
/// # Arguments
/// * `mb` - The MB that shell be converted to byte.
/// # Returns
/// * `u64` -
/// Returns an `u64` containing the number of bytes.
/// # Errors
/// # Example
/// ```
/// let input = 10;
/// let output = mb_in_bytes(input);
/// assert_eq!(output, 10485760);
/// ```
///
pub fn mb_in_bytes(mb: i32) -> u64 {
    (mb * 1024 * 1024) as u64
}

/// Check the integrity of the container.
/// # Arguments
/// * `current_time` - The current time.
/// # Returns
/// * `Result<bool>` -
/// Returns true if the container passed the integrity check otherwise false.
/// In case of an error, this error is returned.
/// # Errors
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// # Example
/// ```
/// let current_time = chrono::Local::now().format("%Y-%m-%dT%H:%M").to_string();
/// let result = check_integrity(&current_time);
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn check_integrity(current_time: &str) -> Result<bool> {
    let output = match Command::new("dmesg").args(["--time-format=iso"]).output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
    }
    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => return Err(SecureContainerErr::ReadingStdoutError(err)),
    };
    let lines: Vec<&str> = stdout.split('\n').collect();

    for line in lines {
        if line.contains("INTEGRITY AEAD ERROR") {
            let time = line.split(' ').collect::<Vec<&str>>()[0];
            let time = time.split(',').collect::<Vec<&str>>()[0];

            if time >= current_time {
                return Ok(false);
            }
        }
    }
    Ok(true)
}

/// Check if integrity check is supported by operating system.
/// # Arguments
/// # Returns
/// * `Result<bool>` -
/// Returns true if the integrity check is supported by the operating system otherwise false.
/// In case of an error, this error is returned.
/// # Errors
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// # Example
/// ```
/// let result = check_functionality_of_integrity();
/// assert_eq!(result.unwrap(), true);
/// ```
///
pub fn check_functionality_of_integrity() -> Result<bool> {
    let output = match Command::new("dmesg").args(["--time-format=iso"]).output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
    }
    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => return Err(SecureContainerErr::ReadingStdoutError(err)),
    };
    let lines: Vec<&str> = stdout.split('\n').collect();

    for line in lines {
        if line.contains("alg: No test for authenc(hmac(sha256),xts(aes)) (authenc(hmac(sha256-avx2),xts-aes-aesni))") {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_check_functionality_of_integrity() {
        let output = check_functionality_of_integrity();
        assert_eq!(output.is_ok(), false);
    }
    #[test]
    fn test_get_password() {
        let input = "test";
        let output = get_password(input);
        //get len
        println!("{:?}", output.unwrap().len());
        //assert_eq!(output.is_ok(), true);
    }

    #[test]
    fn test_convert_to_base64() {
        let input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let output = convert_to_base64(input);
        assert_eq!(output, "AAECAwQFBgcICQ");
    }

    #[test]
    fn test_mb_in_bytes() {
        let input = 10;
        let output = mb_in_bytes(input);
        assert_eq!(output, 10485760);
    }
}
