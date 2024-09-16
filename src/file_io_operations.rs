//! # File I/O Operations
//! This module contains all functions related to file I/O operations.
//! This module is responsible for creating, reading,
//! adding and removing containers from the autoOpen file.
//! The autoOpen file is used for automatically opening containers on startup.
//!

use crate::error_handling;
use error_handling::{check_input, Result, SecureContainerErr};

use crate::file_system_operations::check_if_file_exists;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

/// The path to the autoOpen file.
pub static mut PATH_TO_AUTO_OPEN: &str = "/usr/bin/auto_open";

/// The function that is called to write a new container to the autoOpen file.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(())
/// if the container was added successfully to the auto open file otherwise an error is returned.
/// # Errors
/// * `FileCreationError` - An error occurred while creating a file.
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileWriteError` - An error occurred while writing to a file.
/// # Example
/// ```
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let result = auto_open_write(mount_point, path, namespace, id);
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn auto_open_write(mount_point: &str, path: &str, namespace: &str, id: &str) -> Result<()> {
    let path_to_auto_open = unsafe { PATH_TO_AUTO_OPEN };

    match writing_to_auto_open(mount_point, path, namespace, id, path_to_auto_open) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    Ok(())
}

/// The function that is called to read containers from the autoOpen file.
/// # Arguments
/// # Returns
/// * `Result<Vec<Vec<String>>>` -
/// Returns `Vec<Vec<String>>` with all the data that is needed from all containers that should be opened on startup.
/// If this is not successful, an error is returned.
/// # Errors
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileReadError` - An error occurred while reading a file.
/// # Example
/// ```
/// let sample_data = ["/home/MountMe,/home/Container,MyContainer,myId\n"];
/// let data=[sample_data];
/// let result = auto_open_read();
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn auto_open_read() -> Result<Vec<Vec<String>>> {
    let path_to_auto_open = unsafe { PATH_TO_AUTO_OPEN };

    match reading_auto_open(path_to_auto_open) {
        Ok(containers) => Ok(containers),
        Err(err) => Err(err),
    }
}

/// The internal function that is called to write a new container to the autoOpen file.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// * `path_to_auto_open` - The path to the autoOpen file.
/// # Returns
/// * `Result<()>` -
/// Returns OK(())
/// if the container was added successfully to the auto open file otherwise an error is returned.
/// # Errors
/// * `FileCreationError` - An error occurred while creating a file.
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileWriteError` - An error occurred while writing to a file.
/// # Note
/// This function is not meant to be called directly.
pub fn writing_to_auto_open(
    mount_point: &str,
    path: &str,
    namespace: &str,
    id: &str,
    path_to_auto_open: &str,
) -> Result<()> {
    let data = format!("{},{},{},{}\n", mount_point, path, namespace, id);
    if !check_if_file_exists(path_to_auto_open) {
        let file = File::create(path_to_auto_open);
        if file.is_err() {
            return Err(SecureContainerErr::FileCreationError(
                file.err().unwrap().to_string(),
            ));
        }
    }
    let mut file = match OpenOptions::new().append(true).open(path_to_auto_open) {
        Ok(file) => file,
        Err(err) => return Err(SecureContainerErr::FileOpenError(err.to_string())),
    };
    match file.write_all(data.as_bytes()) {
        Ok(_) => (),
        Err(err) => return Err(SecureContainerErr::FileWriteError(err.to_string())),
    };
    Ok(())
}

/// The function that is called to read containers from the autoOpen file.
/// # Arguments
/// * `path_to_auto_open` - The path to the autoOpen file.
/// # Returns
/// * `Result<Vec<Vec<String>>>` -
/// Returns `Vec<Vec<String>>` with all the data that is needed from all containers that should be opened on startup.
/// If this is not successful, an error is returned.
/// # Errors
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileReadError` - An error occurred while reading a file.
/// # Note
/// This function is not meant to be called directly.
///
pub fn reading_auto_open(path_to_auto_open: &str) -> Result<Vec<Vec<String>>> {
    let mut file = match File::open(path_to_auto_open) {
        Ok(file) => file,
        Err(err) => return Err(SecureContainerErr::FileOpenError(err.to_string())),
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => return Err(SecureContainerErr::FileReadError(err.to_string())),
    };
    let containers: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
    let mut elements: Vec<Vec<String>> = Vec::new();
    for container in containers {
        let element: Vec<String> = container.split(',').map(|s| s.to_string()).collect();
        if element.len() > 1 {
            elements.push(element);
        }
    }
    Ok(elements)
}
/// The function that is called by the daemon to add a new container to the autoOpen file.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(())
/// if the container was added successfully to the auto open file otherwise an error is returned.
/// # Errors
/// * `FileCreationError` - An error occurred while creating a file.
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileWriteError` - An error occurred while writing to a file.
/// ### Errors regarding the input:
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// # Example
/// ```
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let result = auto_open_write(mount_point, path, namespace, id);
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn add_to_auto_open(mount_point: &str, path: &str, namespace: &str, id: &str) -> Result<()> {
    match check_input(
        None,
        Some(mount_point),
        Some(path),
        Some(namespace),
        Some(id),
    ) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match auto_open_write(mount_point, path, namespace, id) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    Ok(())
}

/// The function that is called by the daemon to remove a container from the autoOpen file.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(())
/// if the container was removed successfully from the auto open file otherwise an error is returned.
/// # Errors
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileReadError` - An error occurred while reading a file.
/// * `FileCreationError` - An error occurred while creating a file.
/// * `FileWriteError` - An error occurred while writing to a file.
///
/// ### Errors regarding the input:
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// # Example
/// ```
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let result = remove_auto_open(mount_point, path, namespace, id);
/// assert_eq!(result.is_ok(), true);
/// ```
///
pub fn remove_auto_open(mount_point: &str, path: &str, namespace: &str, id: &str) -> Result<()> {
    let path_to_auto_open = unsafe { PATH_TO_AUTO_OPEN };
    match remove_from_auto_open(mount_point, path, namespace, id, path_to_auto_open) {
        Ok(_) => (),
        Err(err) => panic!("Error removing from auto open: {}", err),
    }
    Ok(())
}

/// The function that is called to remove a container from the autoOpen file.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// * `path_to_auto_open` - The path to the autoOpen file.
/// # Returns
/// * `Result<()>` -
/// Returns OK(())
/// if the container was removed successfully from the auto open file otherwise an error is returned.
/// # Errors
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileReadError` - An error occurred while reading a file.
/// * `FileCreationError` - An error occurred while creating a file.
/// * `FileWriteError` - An error occurred while writing to a file.
///
/// ### Errors regarding the input:
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// # Note
/// This function is not meant to be called directly.
pub fn remove_from_auto_open(
    mount_point: &str,
    path: &str,
    namespace: &str,
    id: &str,
    path_to_auto_open: &str,
) -> Result<()> {
    let containers = match reading_auto_open(path_to_auto_open) {
        Ok(containers) => containers,
        Err(err) => return Err(err),
    };
    let mut new_containers: Vec<Vec<String>> = Vec::new();
    for container in containers {
        if container[0] != mount_point
            && container[1] != path
            && container[2] != namespace
            && container[3] != id
        {
            new_containers.push(container);
        }
    }
    let mut file = match File::create(path_to_auto_open) {
        Ok(file) => file,
        Err(err) => return Err(SecureContainerErr::FileCreationError(err.to_string())),
    };
    for container in new_containers {
        let data = format!(
            "{},{},{},{}\n",
            container[0], container[1], container[2], container[3]
        );
        match file.write_all(data.as_bytes()) {
            Ok(_) => (),
            Err(err) => return Err(SecureContainerErr::FileWriteError(err.to_string())),
        };
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;

    #[test]
    fn test_auto_open_write() {
        let testing_path = "/tmp/auto_open";
        let mount_point = "/mnt";
        let path = "/path";
        let namespace = "namespace";
        let id = "id";
        let data = format!("{},{},{},{}\n", mount_point, path, namespace, id);
        let result = writing_to_auto_open(mount_point, path, namespace, id, testing_path);
        assert_eq!(result.is_ok(), true);
        let mut file = match File::open(testing_path) {
            Ok(file) => file,
            Err(err) => panic!("Error opening file: {}", err),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => panic!("Error reading file: {}", err),
        };
        assert_eq!(contents, data);
        fs::remove_file(testing_path).unwrap();
    }

    #[test]
    fn test_auto_open_read() {
        let testing_path = "/tmp/auto_open2";
        let mount_point = "/mnt";
        let path = "/path";
        let namespace = "namespace";
        let id = "id";
        let data = format!("{},{},{},{}\n", mount_point, path, namespace, id);
        let mut file = match File::create(testing_path) {
            Ok(file) => file,
            Err(err) => panic!("Error creating file: {}", err),
        };
        match file.write_all(data.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!("Error writing to file: {}", err),
        };
        let result = reading_auto_open(testing_path);
        assert_eq!(result.is_ok(), true);
        let result = result.unwrap();
        assert_eq!(result[0][0], mount_point);
        assert_eq!(result[0][1], path);
        assert_eq!(result[0][2], namespace);
        assert_eq!(result[0][3], id);
        fs::remove_file(testing_path).unwrap();
    }

    #[test]
    fn test_remove_from_auto_open() {
        let testing_path = "/tmp/auto_open3";
        let mount_point = "/mnt";
        let path = "/path";
        let namespace = "namespace";
        let id = "id";
        let data = format!("{},{},{},{}\n", mount_point, path, namespace, id);
        let mut file = match File::create(testing_path) {
            Ok(file) => file,
            Err(err) => panic!("Error creating file: {}", err),
        };
        match file.write_all(data.as_bytes()) {
            Ok(_) => (),
            Err(err) => panic!("Error writing to file: {}", err),
        };
        let result = remove_from_auto_open(mount_point, path, namespace, id, testing_path);
        assert_eq!(result.is_ok(), true);
        let mut file = match File::open(testing_path) {
            Ok(file) => file,
            Err(err) => panic!("Error opening file: {}", err),
        };
        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(err) => panic!("Error reading file: {}", err),
        };
        assert_eq!(contents, "");
        fs::remove_file(testing_path).unwrap();
    }
}
