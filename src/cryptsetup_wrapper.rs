//! # Cryptsetup Wrapper
//! This module provides a wrapper for the cryptsetup command line tool.
//! This module is used to create, open, close, export and import a container.
//! It also provides functions to change the password of a container,
//! format a container and check if a file is a LUKS container.
//!
//!

use crate::error_handling;
use error_handling::{check_input, Result, SecureContainerErr};

use crate::file_system_operations;
use file_system_operations::{
    check_container_mounted, check_container_open, check_if_dir_exists, check_if_file_exists,
    check_lsblk, create_file, create_name_dir, mount, unmount,
};

use crate::file_io_operations;
use file_io_operations::auto_open_write;

use crate::utilities;
use utilities::{check_integrity, convert_to_base64, get_password};

use crate::utilities::check_functionality_of_integrity;
use ring::pbkdf2::derive;
use std::io::Write;
use std::num::NonZeroU32;
use std::process::{Command, Stdio};

/// The number of iterations the pseudorandom function for the hmac-sha256 algorithm is executed.
/// This is used for the derivation of the new password for exporting a container.
const COUNT_PSEUDORANDOM_FUNCTION: u32 = 600000; //count for pseudorandom

/// Creates and opens a new container.
/// # Arguments
/// * `size` - The size of the container in MB (must be at least 16MB).
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the directory where the container is stored (must already exist).
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// * `auto_open` -
/// If true,
/// the container is added to the autoOpen file
/// and will be opened automatically when the system starts.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the container was created successfully otherwise an error is returned.
/// # Errors
/// * `FileExists` - A file with the given name already exists in this location.
/// * `ContainerNameExists` - A container with the given name already exists.
/// * `PathNotExists` - The provided path is not a dictionary.
/// * `FileCreationError` - An error occurred while creating a file.
/// * `StdinError` - An error occurred while reading stdin.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// * `ContainerOpen` - The container is already open.
/// * `LibutaDeriveKeyError` - An error occurred while deriving the key.
/// * `LsblkError` - A contaienr with the given name does not exist.
/// * `IntegrityError` - The integrity check failed.
/// * `MkfsError` - An error occurred creation the file system.
/// * `FileOpenError` - An error occurred while opening a file.
/// * `FileWriteError` - An error occurred while writing to a file.
/// * `MountError` - An error occurred while trying to mount the container.
/// ### Errors regarding the input:
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
/// use secure_container::cryptsetup_wrapper;
/// let size = 200;
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let auto_open = true;
/// let result = create_container(size, mount_point, path, namespace, id, auto_open);
/// assert!(result.is_ok());
/// ```
///
pub fn create_container(
    size: i32,
    mount_point: &str,
    path: &str,
    namespace: &str,
    id: &str,
    auto_open: bool,
) -> Result<()> {
    match check_input(
        Some(size),
        Some(mount_point),
        None,
        Some(namespace),
        Some(id),
    ) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    if check_if_file_exists(&(path.to_owned() + "/" + namespace)) {
        return Err(SecureContainerErr::FileExists);
    }
    if check_lsblk(namespace).unwrap() {
        return Err(SecureContainerErr::ContainerNameExists);
    }
    if !check_if_dir_exists(path) {
        return Err(SecureContainerErr::PathNotExists);
    }
    match create_file(size, path, namespace) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    match format_container(&format!("{}/{}", path, namespace), id) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    match check_functionality_of_integrity() {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    if !check_functionality_of_integrity().unwrap() {
        eprintln!("WARNING: Integrity check not supported by operating system!")
    }

    match open_container(
        mount_point,
        &format!("{}/{}", path, namespace),
        namespace,
        id,
    ) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    if auto_open {
        match auto_open_write(mount_point, path, namespace, id) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
    }

    Ok(())
}

/// Open an already existing container.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the container was opened successfully otherwise an error is returned.
/// # Errors
/// * `ContainerOpen` - The container is already open.
/// * `LibutaDeriveKeyError` - An error occurred while deriving the key.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// * `IntegrityError` - The integrity check failed.
/// * `LsblkError` - A contaienr with the given name does not exist.
/// * `MkfsError` - An error occurred creation the file system.
/// * `MountError` - An error occurred while trying to mount the container.
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
/// use secure_container::cryptsetup_wrapper;
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let result = open_container( mount_point, path, namespace, id);
/// assert!(result.is_ok());
/// ```
///
pub fn open_container(mount_point: &str, path: &str, namespace: &str, id: &str) -> Result<()> {
    match check_input(
        None,
        Some(mount_point),
        Some(path),
        Some(namespace),
        Some(id),
    ) {
        Ok(_) => (),
        Err(err) => return Err(err),
    }
    if check_container_open(namespace).unwrap() {
        return Err(SecureContainerErr::ContainerOpen);
    }

    let binding = match get_password(id) {
        Ok(binding) => binding,
        Err(err) => return Err(err),
    };
    let password = binding.as_str();
    let mut child = match Command::new("sudo")
        .args(["cryptsetup", "luksOpen", path, namespace])
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(child) => child,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    {
        let stdin = match child.stdin.as_mut() {
            Some(stdin) => stdin,
            None => {
                return Err(SecureContainerErr::CryptsetupError(
                    "Failed to open stdin".to_string(),
                ))
            }
        };
        let _ = stdin.write_all(password.as_bytes());
    }
    let lsblk = check_lsblk(namespace);

    let output = child.wait_with_output().unwrap();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
    }

    let current_time = chrono::Local::now().format("%Y-%m-%dT%H:%M").to_string();
    let integrity_ok = match check_integrity(&current_time) {
        Ok(integrity) => integrity,
        Err(err) => return Err(err),
    };
    if !integrity_ok {
        let output = match Command::new("sudo")
            .args(["cryptsetup", "luksClose", namespace])
            .output()
        {
            Ok(output) => output,
            Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
        };
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
        }
        return Err(SecureContainerErr::IntegrityError);
    }
    if !lsblk.unwrap() {
        match create_name_dir(namespace) {
            Ok(_) => (),
            Err(err) => return Err(err),
        };
    }

    match mount(mount_point, namespace) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    Ok(())
}

/// Close an already existing container that is open.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `namespace` - The name of the container.
///
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the container was closed successfully otherwise an error is returned.///
/// # Errors
/// * `UmountError` - An error occurred while the container was unmounted.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
///
/// ### Errors regarding the input:
/// * `MountPointNotExists` - The given mount point does not exist.
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// # Example
/// ```
/// use secure_container::cryptsetup_wrapper;
/// let mount_point = "/home/MountMe";
/// let namespace = "MyContainer";
/// let result = close_container(mount_point, namespace);
/// assert!(result.is_ok());
/// ```
///
pub fn close_container(mount_point: &str, namespace: &str) -> Result<()> {
    match check_input(None, Some(mount_point), None, Some(namespace), None) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    match unmount(mount_point) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    let output = match Command::new("sudo")
        .args(["cryptsetup", "luksClose", namespace])
        .output()
    {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
    }
    Ok(())
}

/// Exporting an existing and closed container.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// * `secret` - The secret for the container (is needed when container is imported).
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the container was exported successfully otherwise an error is returned.
/// # Errors
/// * `LsblkError` - A contaienr with the given name does not exist.
/// * `ReadingStdoutError` - An error occurred while reading stdout.
/// * `ContainerOpen` - The container is already open.
/// * `LsError` - An error occurred while checking the logical volumes of the system.
/// * `ContainerMounted` - The container is still mounted.
/// * `LibutaDeriveKeyError` - An error occurred while deriving the key.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// ### Errors regarding the input:
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// * `SecertError` - The secret is empty or contains non-ascii characters.
/// # Example
/// ```
/// use secure_container::cryptsetup_wrapper;
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let secret = "mySecret";
/// let result = export_container(mount_point, path, namespace, id, secret);
/// assert!(result.is_ok());
/// ```
///
pub fn export_container(path: &str, namespace: &str, id: &str, secret: &str) -> Result<()> {
    match check_input(None, None, Some(path), Some(namespace), Some(id)) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    if secret.is_empty() {
        return Err(SecureContainerErr::SecertError);
    }
    if !secret.is_ascii() {
        return Err(SecureContainerErr::SecertError);
    }
    if match check_container_open(namespace) {
        Ok(true) => true,
        Ok(false) => false,
        Err(err) => return Err(err),
    } {
        return Err(SecureContainerErr::ContainerOpen);
    }

    if match check_container_mounted(namespace) {
        Ok(true) => true,
        Ok(false) => false,
        Err(err) => return Err(err),
    } {
        return Err(SecureContainerErr::ContainerMounted);
    }

    //hash secret
    let mut out = [0u8; 32];
    derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(COUNT_PSEUDORANDOM_FUNCTION).unwrap(),
        secret.as_bytes(),
        namespace.as_bytes(),
        &mut out,
    );

    let password = convert_to_base64(out.to_vec());

    let old_password = match get_password(id) {
        Ok(old_password) => old_password,
        Err(err) => return Err(err),
    };

    match change_password(path, &old_password, &password) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    Ok(())
}

/// Importing an existing container.
/// # Arguments
/// * `mount_point` - The path to the mount point (must already exist).
/// * `path` - The path to the container.
/// * `namespace` - The name of the container.
/// * `id` - The id of the container.
/// * `secret` - The secret for the container (is needed when container is imported).
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the container was imported successfully otherwise an error is returned.
/// # Errors
/// * `LibutaDeriveKeyError` - An error occurred while deriving the key.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// ### Errors regarding the input:
/// * `NamespaceNotValid` - The given namespace contains non-ascii characters or a pipe.
/// * `IdNotValid` - The given id contains non-ascii characters, a pipe or is longer than 8 characters.
/// * `PathNotValid` - The given path contains non-ascii characters or a pipe.
/// * `PathNotExists` - The given path does not exist.
/// * `PathNotLuksContainer` - The given path is not a LUKS container.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// * `SecertError` - The secret is empty or contains non-ascii characters.
/// # Example
/// ```
/// use secure_container::cryptsetup_wrapper;
/// let mount_point = "/home/MountMe";
/// let path = "/home/Container";
/// let namespace = "MyContainer";
/// let id = "myId";
/// let secret = "mySecret";
/// let result = import_container(mount_point, path, namespace, id, secret);
/// assert!(result.is_ok());
/// ```
///
pub fn import_container(path: &str, namespace: &str, id: &str, secret: &str) -> Result<()> {
    match check_input(None, None, Some(path), Some(namespace), Some(id)) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };

    //hash secret
    let mut out = [0u8; 32];
    derive(
        ring::pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(COUNT_PSEUDORANDOM_FUNCTION).unwrap(),
        secret.as_bytes(),
        namespace.as_bytes(),
        &mut out,
    );

    let password = convert_to_base64(out.to_vec());
    let password_new = match get_password(id) {
        Ok(old_password) => old_password,
        Err(err) => return Err(err),
    };
    //change password from container
    match change_password(path, &password, &password_new) {
        Ok(_) => (),
        Err(err) => return Err(err),
    };
    Ok(())
}

/// Change the password of an existing container.
/// # Arguments
/// * `path` - The path to the container.
/// * `password_old` - The old password of the container.
/// * `password` - The new password of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the password was changed successfully otherwise an error is returned.
/// # Errors
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// # Example
/// ```
/// use secure_container::cryptsetup_wrapper;
/// let path = "/home/Container";
/// let old_password = "myOldPassword";
/// let new_password = "myNewPassword";
/// let result = change_password(path, old_password, new_password);
/// assert!(result.is_ok());
/// ```
///
fn change_password(path: &str, old_password: &str, password: &str) -> Result<()> {
    let mut output = match Command::new("/usr/sbin/cryptsetup")
        .args(["luksChangeKey", path])
        .stdin(Stdio::piped())
        .spawn()
    {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };

    let stdin = match output.stdin.as_mut() {
        Some(stdin) => stdin,
        None => {
            return Err(SecureContainerErr::CryptsetupError(
                "Failed to open stdin".to_string(),
            ))
        }
    };

    let _ = stdin.write_all(old_password.as_bytes());
    let _ = stdin.write_all(b"\n");
    let _ = stdin.write_all(password.as_bytes());

    let done = output.wait_with_output().unwrap();
    if !done.status.success() {
        let stderr = String::from_utf8_lossy(&done.stderr);
        return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
    }
    Ok(())
}

/// Checks if the provided file is a LUKS container.
/// # Arguments
/// * `path` - The path to the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the file is a LUKS container, otherwise an error is returned.
/// # Errors
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// * `IsNotLuks` - The provided file is not a LUKS container.
/// # Example
/// ```
/// use secure_container::cryptsetup_wrapper;
/// let path = "/home/Container";
/// let result = check_if_file_is_container(path);
/// assert!(result.is_ok());
/// ```
///
pub fn check_if_file_is_container(path: &str) -> Result<()> {
    let output = match Command::new("/usr/sbin/cryptsetup")
        .args(["isLuks", path])
        .spawn()
    {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    let done = output.wait_with_output().unwrap();
    if !done.status.success() {
        let stderr = String::from_utf8_lossy(&done.stderr);
        return Err(SecureContainerErr::IsNotLuks(stderr.to_string()));
    }
    Ok(())
}

/// Formats a LUKS container.
/// # Arguments
/// * `device_path` - The path to the file that will be the LUKS container.
/// * `id` - The id of the container.
/// # Returns
/// * `Result<()>` -
/// Returns OK(()) if the container was formatted successfully otherwise an error is returned.
/// # Errors
/// * `StdinError` - An error occurred while reading stdin.
/// * `CryptsetupError` - An error occurred while executing the cryptsetup command.
/// # Example
/// ```
/// use secure_container::cryptsetup_wrapper;
/// let device_path = "/home/Container";
/// let id = "myId";
/// let result = format_container(size, mount_point, path, namespace, id, auto_open);
/// assert!(result.is_ok());
/// ```
///
fn format_container(device_path: &str, id: &str) -> Result<()> {
    let bind = get_password(id);
    if bind.is_err() {
        return Err(SecureContainerErr::StdinError(
            "Error getting password".to_string(),
        ));
    }
    let bind = bind.unwrap();
    let password = bind.as_str();

    let mut output = match Command::new("/usr/sbin/cryptsetup")
        .args([
            "luksFormat",
            device_path,
            "--type",
            "luks2",
            "--integrity",
            "hmac-sha256",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    {
        let stdin = match output.stdin.as_mut() {
            Some(stdin) => stdin,
            None => {
                return Err(SecureContainerErr::CryptsetupError(
                    "Failed to open stdin".to_string(),
                ))
            }
        };
        let _ = stdin.write_all(password.as_bytes());
    }

    let done = match output.wait_with_output() {
        Ok(output) => output,
        Err(err) => return Err(SecureContainerErr::CryptsetupError(err.to_string())),
    };
    if !done.status.success() {
        let stderr = String::from_utf8_lossy(&done.stderr);
        return Err(SecureContainerErr::CryptsetupError(stderr.to_string()));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{export_container, SecureContainerErr};
    use std::any::Any;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_functionality() {
        //get a current path
        let mut current_path = std::env::current_dir().unwrap();

        //create folder for testing
        current_path.push("Testing");
        let path_container = current_path.to_str().unwrap();
        let mut path = Path::new(&current_path);
        if !path.exists() {
            fs::create_dir(path).unwrap();
        }

        //Create Mount Point
        let mut current_path = std::env::current_dir().unwrap();
        current_path.push("Testing");
        current_path.push("MountME");
        let mount_point = current_path.to_str().unwrap();
        path = Path::new(&current_path);
        if !path.exists() {
            fs::create_dir(path).unwrap();
        }

        let size = 200;
        let namespace = "ThisIsAContainerForTestingPurposes";
        let id = "test";
        let auto_open = true;
        let binding = format!("{}/{}", path_container, namespace);
        let path_to_container = binding.as_str();
        let secret = "123";
        //setupt test enviornment

        print_blogs("Test Create Container");
        test_create_container_wrong_input(
            size,
            mount_point,
            path_container,
            namespace,
            id,
            auto_open,
        );

        print_blogs("Test Open Container");
        test_open_container_wrong_input(mount_point, path_to_container, namespace, id);

        print_blogs("Test Close Container");
        test_close_container_wrong_input(namespace, mount_point);

        print_blogs("Test Export Container");
        test_export_container_wrong_input(path_to_container, namespace, id, "");

        print_blogs("Test Import Container");
        test_import_container_wrong_input(path_to_container, namespace, id, "");
        test_import_container_wrong_secret(path_to_container, namespace, id, secret);
    }

    fn print_blogs(message: &str) {
        println!("##############################################################################################################");
        println!("{}", message.to_uppercase());
        println!("##############################################################################################################");
    }

    fn test_create_container_wrong_input(
        size: i32,
        mount_point: &str,
        path: &str,
        namespace: &str,
        id: &str,
        auto_open: bool,
    ) {
        let result_size = super::create_container(15, mount_point, path, namespace, id, auto_open);
        let result_mountpoint = super::create_container(
            size,
            "/wqsedrftgzhuiizurfcgjhg",
            "/home/tian/test",
            namespace,
            id,
            auto_open,
        );
        let result_path = super::create_container(
            size,
            mount_point,
            "/rtcfvgbuzhnijkm",
            namespace,
            id,
            auto_open,
        );
        let result_namespace =
            super::create_container(size, mount_point, path, "test|", id, auto_open);
        let result_namespace_non_ascii =
            super::create_container(size, mount_point, path, "test¢", id, auto_open);
        let result_id =
            super::create_container(size, mount_point, path, namespace, "test|", auto_open);
        let result_id_non_ascii =
            super::create_container(size, mount_point, path, namespace, "test¢", auto_open);
        let result_id_to_long =
            super::create_container(size, mount_point, path, namespace, "testtest", auto_open);

        assert_eq!(result_size.err().unwrap(), SecureContainerErr::SizeToSmall);
        assert_eq!(
            result_mountpoint.err().unwrap(),
            SecureContainerErr::MountPointNotExists
        );
        assert_eq!(
            result_path.err().unwrap(),
            SecureContainerErr::PathNotExists
        );
        assert_eq!(
            result_namespace.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(
            result_namespace_non_ascii.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(result_id.err().unwrap(), SecureContainerErr::IdNotValid);
        assert_eq!(
            result_id_non_ascii.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
        assert_eq!(
            result_id_to_long.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
    }

    fn test_open_container_wrong_input(mount_point: &str, path: &str, namespace: &str, id: &str) {
        let result_mountpoint = super::open_container("/home/tian/test12345", path, namespace, id);
        let result_path = super::open_container(mount_point, "/home/tian/test12345", namespace, id);
        let result_namespace = super::open_container(mount_point, path, "test|", id);
        let result_namespace_non_ascii = super::open_container(mount_point, path, "test¢", id);
        let result_id = super::open_container(mount_point, path, namespace, "test|");
        let result_id_non_ascii = super::open_container(mount_point, path, namespace, "test¢");
        let result_id_to_long = super::open_container(mount_point, path, namespace, "testtest");
        assert_eq!(
            result_mountpoint.err().unwrap(),
            SecureContainerErr::MountPointNotExists
        );
        assert_eq!(
            result_path.err().unwrap(),
            SecureContainerErr::PathNotExists
        );
        assert_eq!(
            result_namespace.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(
            result_namespace_non_ascii.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(result_id.err().unwrap(), SecureContainerErr::IdNotValid);
        assert_eq!(
            result_id_non_ascii.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
        assert_eq!(
            result_id_to_long.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
    }

    fn test_close_container_wrong_input(container_name: &str, mount_point: &str) {
        let result_mountpoint = super::close_container("/home/tian/test12345", container_name);
        let result_namespace = super::close_container(mount_point, "test|");
        let result_namespace_non_ascii = super::close_container(mount_point, "test¢");
        let result_container_not_open = super::close_container(mount_point, "test");
        assert_eq!(
            result_mountpoint.err().unwrap(),
            SecureContainerErr::MountPointNotExists
        );
        assert_eq!(
            result_namespace.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(
            result_namespace_non_ascii.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(
            result_container_not_open.err().unwrap().type_id(),
            SecureContainerErr::UmountError("A".to_string()).type_id()
        );
    }

    fn test_export_container_wrong_input(path: &str, namespace: &str, id: &str, secret: &str) {
        let result_path = export_container("/home/tian/MountME", namespace, id, secret);
        let result_namespace = export_container(path, "test|", id, secret);
        let result_namespace_non_ascii = export_container(path, "test¢", id, secret);
        let result_id = export_container(path, namespace, "test|", secret);
        let result_id_non_ascii = export_container(path, namespace, "test¢", secret);
        let result_id_to_long = export_container(path, namespace, "testtest", secret);
        let result_id_wrong = export_container(path, namespace, "1234", secret);
        let result_secret_empty = export_container(path, namespace, id, "");
        let result_secert_non_ascii = export_container(path, namespace, id, "test¢");
        assert_eq!(
            result_path.err().unwrap(),
            SecureContainerErr::PathNotExists
        );
        assert_eq!(
            result_namespace.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(
            result_namespace_non_ascii.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(result_id.err().unwrap(), SecureContainerErr::IdNotValid);
        assert_eq!(
            result_id_non_ascii.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
        assert_eq!(
            result_id_to_long.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
        assert_eq!(
            result_id_wrong.err().unwrap().type_id(),
            SecureContainerErr::StdinError("Error getting password".to_string()).type_id()
        );
        assert_eq!(
            result_secret_empty.err().unwrap().type_id(),
            SecureContainerErr::SecertError.type_id()
        );
        assert_eq!(
            result_secert_non_ascii.err().unwrap().type_id(),
            SecureContainerErr::SecertError.type_id()
        );
    }

    fn test_import_container_wrong_input(path: &str, namespace: &str, id: &str, secret: &str) {
        let result_path = super::import_container("/home/tian/MountME", namespace, id, secret);
        let result_namespace = super::import_container(path, "test|", id, secret);
        let result_namespace_non_ascii = super::import_container(path, "test¢", id, secret);
        let result_id = super::import_container(path, namespace, "test|", secret);
        let result_id_non_ascii = super::import_container(path, namespace, "test¢", secret);
        let result_id_to_long = super::import_container(path, namespace, "testtest", secret);
        let result_id_wrong = super::import_container(path, namespace, "1234", secret);
        let result_secret_empty = super::import_container(path, namespace, id, "");
        let result_secret_non_ascii = super::import_container(path, namespace, id, "test¢");
        assert_eq!(
            result_path.err().unwrap(),
            SecureContainerErr::PathNotExists
        );
        assert_eq!(
            result_namespace.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(
            result_namespace_non_ascii.err().unwrap(),
            SecureContainerErr::NamespaceNotValid
        );
        assert_eq!(result_id.err().unwrap(), SecureContainerErr::IdNotValid);
        assert_eq!(
            result_id_non_ascii.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
        assert_eq!(
            result_id_to_long.err().unwrap(),
            SecureContainerErr::IdNotValid
        );
        assert_eq!(
            result_id_wrong.err().unwrap().type_id(),
            SecureContainerErr::StdinError("Error getting password".to_string()).type_id()
        );
        assert_eq!(
            result_secret_empty.err().unwrap().type_id(),
            SecureContainerErr::SecertError.type_id()
        );
        assert_eq!(
            result_secret_non_ascii.err().unwrap().type_id(),
            SecureContainerErr::SecertError.type_id()
        );
    }
    fn test_import_container_wrong_secret(path: &str, namespace: &str, id: &str, secret: &str) {
        let result = super::import_container(path, namespace, id, secret);
        assert_eq!(
            result.err().unwrap().type_id(),
            SecureContainerErr::CryptsetupError("".to_string()).type_id()
        );
    }
}
