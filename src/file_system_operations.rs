//! # File System Operations
//! This module provides all function
//! needed to interact with the file system for the entire project.
//! It provides functions for checking file or directory,
//! creating files and directories, checking if a container is mounted,
//! creating a directory for the container,
//! mounting and unmounting the container, and checking if the container is open.
//!

use crate::error_handling;
use error_handling::{Result, SecureContainerErr};

use crate::utilities;
use utilities::mb_in_bytes;

use std::fs::File;
use std::io::Write;

use std::path::Path;
use std::process::Command;

/// Check if a file exists
/// # Arguments
/// * `path` - The path to a file.
/// # Returns
/// * `bool` - True if the provided path is a file otherwise false.
/// In case of an error, this error is returned.
/// # Example
/// ```
/// let path = "/usr/bin/auto_open";
/// let result = check_if_file_exists(path);
/// assert_eq!(result, true);
/// ```
///
pub fn check_if_file_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.is_file()
}

/// Check if a directory exists
/// # Arguments
/// * `path` - The path to a directory.
/// # Returns
/// * `bool` - True if the provided path is a directory otherwise false.
/// In case of an error, this error is returned.
/// # Example
/// ```
/// let path = "/usr/bin";
/// let result = check_if_file_exists(path);
/// assert_eq!(result, true);
/// ```
///
pub fn check_if_dir_exists(path: &str) -> bool {
    let path = Path::new(path);
    path.is_dir()
}

/// Create a file
/// # Arguments
/// * `size` - Filesize in MB.
/// * `path` - The path to where the file should be created.
/// * `namespace` - The name of the file.
/// # Returns
/// * `Result<()>` -
/// Returns OK(())
/// if the file was created successfully otherwise an error is returned.
/// # Errors
/// * `FileCreationError` - An error occurred while creating a file.
/// * `FileWriteError` - An error occurred while writing to a file.
/// # Example
/// ```
/// let size = 10;
/// let path = "/usr/bin";
/// let namespace = "test.txt";
/// let result = create_file(size, path, namespace);
/// assert!(result.is_ok());
/// ```
///
pub fn create_file(size: i32, path: &str, namespace: &str) -> Result<()> {
    let complete_path = Path::new(path).join(namespace);
    let file_size_in_bytes = mb_in_bytes(size);
    let mut file = match File::create(complete_path) {
        Ok(file) => file,
        Err(err) => return Err(SecureContainerErr::FileCreationError(err.to_string())),
    };

    let mut bytes_written = 0;
    while bytes_written < file_size_in_bytes {
        let bytes_to_write = std::cmp::min(1024, file_size_in_bytes - bytes_written) as usize;
        let data = vec![0u8; bytes_to_write];
        match file.write_all(&data) {
            Ok(_) => bytes_written += bytes_to_write as u64,
            Err(err) => return Err(SecureContainerErr::FileWriteError(err.to_string())),
        };
    }

    Ok(())
}

/// Check connected block devices using lsblk
/// # Arguments
/// * `name` - The name of the block device.
/// # Returns
/// * `Result<bool>` -
/// Returns true if the block device is connected otherwise false.
/// In case of an error, this error is returned.
/// # Errors
/// * `LsblkError` - An error occurred executing lsblk.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// # Example
/// ```
/// let name = "myBlockDevice";
/// let result = check_lsblk(name);
/// assert_eq!(result.unwrap(), true);
/// ```
///
pub fn check_lsblk(name: &str) -> Result<bool> {
    let output = match Command::new("lsblk").output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::LsblkError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::LsblkError(stderr.to_string()));
    }
    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => return Err(SecureContainerErr::ReadingStdoutError(err)),
    };
    let lines: Vec<&str> = stdout.split(' ').collect();
    for line in lines {
        let mut line = line.replace('\n', "");
        line = line.replace("└─", "");
        if line == name {
            return Ok(true);
        }
    }

    Ok(false)
}

/// Check if a container is mounted
/// # Arguments
/// * `namespace` - The name of the container.
/// # Returns
/// * `Result<bool>` -
/// Returns true if the container is mounted otherwise false.
/// In case of an error, this error is returned.
/// # Errors
/// * `LsError` - An error occurred while checking the logical volumes of the system.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// # Example
/// ```
/// let namespace = "myContainer";
/// let result = check_container_mounted(namespace);
/// assert_eq!(result.unwrap(), true);
/// ```
///
pub fn check_container_mounted(namespace: &str) -> Result<bool> {
    let output = match Command::new("ls").args(["-l", "/dev/mapper"]).output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::LsError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::LsError(stderr.to_string()));
    }
    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => return Err(SecureContainerErr::ReadingStdoutError(err)),
    };
    let lines: Vec<&str> = stdout.split('\n').collect();
    for line in lines {
        if line.contains(&format!("{} ", namespace)) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Create a directory for the container in /dev/mapper
/// # Arguments
/// * `namespace` - The name of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the directory was created successfully otherwise an error is returned.
/// # Errors
/// * `MkfsError` - An error occurred creation the file system.
/// # Example
/// ```
/// let namespace = "myContainer";
/// let result = create_name_dir(namespace);
/// assert!(result.is_ok());
/// ```
///
pub fn create_name_dir(namespace: &str) -> Result<()> {
    let path = Path::new("/dev/mapper");
    let file_path = path.join(namespace);

    let output = match Command::new("/sbin/mkfs.ext4").args(&[file_path]).output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::MkfsError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::MkfsError(stderr.to_string()));
    }

    Ok(())
}

/// Mount a device to a directory
/// # Arguments
/// * `mount_point` - The directory where the device should be mounted to.
/// * `device` - The name of the device to be mounted.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the device was mounted successfully otherwise an error is returned.
/// # Errors
/// * `MountError` - An error occurred while trying to mount the container.
/// # Example
/// ```
/// let mount_point = "/home/MountMe";
/// let device = "myContainer";
/// let result = mount(mount_point, device);
/// assert!(result.is_ok());
/// ```
///
pub fn mount(mount_point: &str, device: &str) -> Result<()> {
    let binding = "/dev/mapper/".to_owned() + device;
    let device = binding.as_str();
    let output = match Command::new("mount").args([device, mount_point]).output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::MountError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::MountError(stderr.to_string()));
    }

    Ok(())
}

/// Unmount a device from a directory
/// # Arguments
/// * `mount_point` - The directory where the device is mounted to.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the device was unmounted successfully otherwise an error is returned.
/// # Errors
/// * `UmountError` - An error occurred while the device was unmounted.
/// # Example
/// ```
/// let mount_point = "/home/MountMe";
/// let result = unmount(mount_point);
/// assert!(result.is_ok());
/// ```
///
pub fn unmount(mount_point: &str) -> Result<()> {
    let output = match Command::new("umount").args([mount_point]).output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::UmountError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::UmountError(stderr.to_string()));
    }
    Ok(())
}

/// Check if a container is open
/// # Arguments
/// * `namespace` - The name of the container.
/// # Returns
/// * `Result<bool>` -
/// Returns true if the container is open otherwise false.
/// In case of an error, this error is returned.
/// # Errors
/// * `LsblkError` - An error occurred executing lsblk.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// # Example
/// ```
/// let namespace = "myContainer";
/// let result = check_container_open(namespace);
/// assert_eq!(result.unwrap(), false);
/// ```
///

pub fn check_container_open(namespace: &str) -> Result<bool> {
    let output = match Command::new("lsblk")
        .args(["-o", "NAME,TYPE,MOUNTPOINT"])
        .output()
    {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::LsblkError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::LsblkError(stderr.to_string()));
    }

    let stdout = match String::from_utf8(output.stdout) {
        Ok(stdout) => stdout,
        Err(err) => return Err(SecureContainerErr::ReadingStdoutError(err)),
    };
    let lines: Vec<&str> = stdout.split('\n').collect();
    for line in lines {
        if line.contains(&format!("{} ", namespace)) && line.contains("crypt") {
            return Ok(true);
        }
    }
    Ok(false)
}
