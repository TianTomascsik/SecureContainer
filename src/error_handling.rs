//! # Error handling
//! This module contains all the error definitions and input validation for the whole project.
//! Here is the custom error type `SecureContainerErr` and a custom result type `Result<E>` defined.
//!
use crate::file_system_operations;
use file_system_operations::{check_if_dir_exists, check_if_file_exists};

use crate::cryptsetup_wrapper;
use cryptsetup_wrapper::check_if_file_is_container;

use std::{fmt, string};
/// The `Result<E>` type is used to return the custom error type from functions.
pub type Result<E> = std::result::Result<E, SecureContainerErr>;
/// The `SecureContainerErr`
/// type is an enum that defines all possible errors that can occur in the project.
#[derive(Debug, PartialEq)]
pub enum SecureContainerErr {
    SizeToSmall,
    MountPointNotExists,
    PathNotExists,
    NamespaceNotValid,
    IdNotValid,
    LsblkError(String),
    ReadingStdoutError(string::FromUtf8Error),
    UmountError(String),
    MountError(String),
    MkfsError(String),
    LsError(String),
    CryptsetupError(String),
    StdinError(String),
    FileCreationError(String),
    FileWriteError(String),
    LibutaDeriveKeyError(String),
    FileReadError(String),
    FileOpenError(String),
    IntegrityError,
    ContainerMounted,
    ContainerOpen,
    ContainerNameExists,
    FileExists,
    SecertError,
    PathNotLuksContainer,
    PathNotValid,
    IsNotLuks(String),
    OK,
}
/// Here the `Display` trait for the costem `SecureContainerErr` type is implemented.
/// # Example
/// ```
/// use secure_container::error_handling::{SecureContainerErr};
/// let err = SecureContainerErr::SizeToSmall;
/// println!("{}", err);
/// ```
impl fmt::Display for SecureContainerErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SecureContainerErr::SizeToSmall => write!(f, "Size of container to small"),
            SecureContainerErr::MountPointNotExists => write!(f, "Mountpoint wrong"),
            SecureContainerErr::PathNotExists => write!(f, "Not valid path"),
            SecureContainerErr::NamespaceNotValid => write!(f, "Not valid namespace"),
            SecureContainerErr::IdNotValid => write!(f, "Not valid id"),
            SecureContainerErr::LsblkError(err) => write!(f, "Lsblk error: {}", err),
            SecureContainerErr::ReadingStdoutError(err) => {
                write!(f, "Reading stdout error: {}", err)
            }
            SecureContainerErr::UmountError(err) => write!(f, "Umount error: {}", err),
            SecureContainerErr::MountError(err) => write!(f, "Mount error: {}", err),
            SecureContainerErr::MkfsError(err) => write!(f, "Mkfs error: {}", err),
            SecureContainerErr::LsError(err) => write!(f, "Ls error: {}", err),
            SecureContainerErr::CryptsetupError(err) => write!(f, "Cryptsetup error: {}", err),
            SecureContainerErr::StdinError(err) => write!(f, "Stdin error: {}", err),
            SecureContainerErr::FileCreationError(err) => write!(f, "File creation error: {}", err),
            SecureContainerErr::FileWriteError(err) => write!(f, "File write error: {}", err),
            SecureContainerErr::LibutaDeriveKeyError(err) => write!(f, "Libuta derive key error: {}",err),
            SecureContainerErr::FileReadError(err) => write!(f, "File read error: {}", err),
            SecureContainerErr::FileOpenError(err) => write!(f, "File open error: {}", err),
            SecureContainerErr::IntegrityError => write!(f, "Integrity error"),
            SecureContainerErr::ContainerMounted => write!(f, "Container mounted"),
            SecureContainerErr::ContainerOpen => write!(f, "Container open"),
            SecureContainerErr::ContainerNameExists => {
                write!(f, "Container with that name already exists")
            }
            SecureContainerErr::FileExists => write!(f, "File already exists"),
            SecureContainerErr::SecertError => write!(f, "Secret not valid"),
            SecureContainerErr::PathNotLuksContainer => write!(f, "Path is not a luks container"),
            SecureContainerErr::PathNotValid => write!(f, "Path not valid"),
            SecureContainerErr::IsNotLuks(err) => write!(f, "Path is not a luks divice: {}", err),
            SecureContainerErr::OK => write!(f, "OK"),
        }
    }
}

/// Checks the given input if they are valid and can be used further by different functions.
/// # Arguments
/// * `size` - The size of the container in MB (must be at least 16MB).
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the provided inputs are valid otherwise an error is returned.
/// # Errors
/// * `SizeToSmall` - The given size for the container is too small.
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// # Example
/// ```
/// use secure_container::error_handling::{check_input};
/// let size = 12;
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let result = check_input(Some(size), Some(mount_point), Some(path), Some(namespace), Some(id));
/// assert_eq!(result, Err(SecureContainerErr::SizeToSmall));
/// ```
///

pub fn check_input(
    size: Option<i32>,
    mount_point: Option<&str>,
    path: Option<&str>,
    namespace: Option<&str>,
    id: Option<&str>,
) -> Result<()> {
    if size.is_some() && size.unwrap() < 16 {
        return Err(SecureContainerErr::SizeToSmall);
    }

    if mount_point.is_some() && !check_if_dir_exists(mount_point.unwrap()) {
        return Err(SecureContainerErr::MountPointNotExists);
    }

    if namespace.is_some() && (!namespace.unwrap().is_ascii() || namespace.unwrap().contains('|')) {
        return Err(SecureContainerErr::NamespaceNotValid);
    }

    if id.is_some()
        && (id.unwrap().contains('|') || !id.unwrap().is_ascii() || id.unwrap().len() >= 8)
    {
        return Err(SecureContainerErr::IdNotValid);
    }

    if path.is_some() && (!path.unwrap().is_ascii() || path.unwrap().contains('|')) {
        return Err(SecureContainerErr::PathNotValid);
    }

    if path.is_some() && !check_if_file_exists(path.unwrap()) {
        return Err(SecureContainerErr::PathNotExists);
    }
    if path.is_some() && check_if_file_is_container(path.unwrap()).is_err() {
        return Err(SecureContainerErr::PathNotLuksContainer);
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::error_handling::SecureContainerErr::CryptsetupError;
    use std::fs::File;

    #[test]
    fn test_check_input() {
        let path = std::env::current_dir().unwrap();
        let _file = File::create(path.join("test.txt"));
        let mount_point = path.to_str().unwrap();
        let path = path.join("test.txt");
        let path = path.to_str().unwrap();
        let namespace = "test";
        let id = "test";
        let size = 16;
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some("not_exists"),
                Some(namespace),
                Some(id)
            ),
            Err(SecureContainerErr::PathNotExists)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some("not_ascii€"),
                Some(namespace),
                Some(id)
            ),
            Err(SecureContainerErr::PathNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some("contains|"),
                Some(namespace),
                Some(id)
            ),
            Err(SecureContainerErr::PathNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some(path),
                Some(namespace),
                Some(id)
            ),
            Err(SecureContainerErr::PathNotLuksContainer)
        );
        assert_eq!(
            check_input(
                Some(15),
                Some(mount_point),
                Some(path),
                Some(namespace),
                Some(id)
            ),
            Err(SecureContainerErr::SizeToSmall)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some("not_exists"),
                Some(path),
                Some(namespace),
                Some(id)
            ),
            Err(SecureContainerErr::MountPointNotExists)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some(path),
                Some("test|"),
                Some(id)
            ),
            Err(SecureContainerErr::NamespaceNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some(path),
                Some("not_ascii€"),
                Some(id)
            ),
            Err(SecureContainerErr::NamespaceNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some(path),
                Some(namespace),
                Some("test€")
            ),
            Err(SecureContainerErr::IdNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some(path),
                Some(namespace),
                Some("test|")
            ),
            Err(SecureContainerErr::IdNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                Some(path),
                Some(namespace),
                Some("testtest")
            ),
            Err(SecureContainerErr::IdNotValid)
        );
        assert_eq!(
            check_input(
                Some(size),
                Some(mount_point),
                None,
                Some(namespace),
                Some("test")
            ),
            Ok(())
        );
        let _ = std::fs::remove_file(path);
    }
    #[test]
    fn test_fmt() {
        let bytes = vec![0, 159];
        let value = String::from_utf8(bytes);
        let test = value.unwrap_err();
        let error_list = [
            CryptsetupError("test".to_string()),
            SecureContainerErr::OK,
            SecureContainerErr::SizeToSmall,
            SecureContainerErr::MountPointNotExists,
            SecureContainerErr::PathNotExists,
            SecureContainerErr::NamespaceNotValid,
            SecureContainerErr::IdNotValid,
            SecureContainerErr::PathNotValid,
            SecureContainerErr::PathNotLuksContainer,
            SecureContainerErr::IsNotLuks("test".to_string()),
            SecureContainerErr::LsblkError("test".to_string()),
            SecureContainerErr::ReadingStdoutError(test),
            SecureContainerErr::UmountError("test".to_string()),
            SecureContainerErr::MountError("test".to_string()),
            SecureContainerErr::MkfsError("test".to_string()),
            SecureContainerErr::LsError("test".to_string()),
            SecureContainerErr::CryptsetupError("test".to_string()),
            SecureContainerErr::StdinError("test".to_string()),
            SecureContainerErr::FileCreationError("test".to_string()),
            SecureContainerErr::FileWriteError("test".to_string()),
            SecureContainerErr::LibutaDeriveKeyError("test".to_string()),
            SecureContainerErr::FileReadError("test".to_string()),
            SecureContainerErr::FileOpenError("test".to_string()),
            SecureContainerErr::IntegrityError,
            SecureContainerErr::ContainerMounted,
            SecureContainerErr::ContainerOpen,
            SecureContainerErr::ContainerNameExists,
            SecureContainerErr::FileExists,
            SecureContainerErr::SecertError,
            SecureContainerErr::PathNotLuksContainer,
            SecureContainerErr::PathNotValid,
        ];
        for error in error_list.iter() {
            println!("{}", error);
        }
    }
}
