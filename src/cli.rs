//! # CLI
//!
//! The CLI (command line interface) is a sample application that provides an overview
//! of how to use the secure container service.
//!
//! ## Usage
//! Structured as follows:
//! ```bash
//! secure_container_cli [SUBCOMMAND] [OPTIONS]
//! ```
//! The following subcommands are available:
//! ### Create
//! This is a subcommand to create a new Container.
//!
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli create [OPTIONS] <SIZE> <MOUNT_POINT> <PATH> <NAMESPACE> <ID>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <SIZE>         Size of the container in MB (at least 16MB)
//!   <MOUNT_POINT>  Mount point of the container
//!   <PATH>         Path where the container should be stored
//!   <NAMESPACE>    Name of the container
//!   <ID>           ID of the container (max 8 characters)
//! ```
//! <u> Options: </u>
//! ```bash
//!  -a, --auto-open   To add the container to the AutoOpen file so that it is automatically opened when the system starts.
//!  -h, --help        Print help
//! ```
//!
//! ### Open
//! This is a subcommand to open an existing Container.
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli open <MOUNT_POINT> <PATH> <NAMESPACE> <ID>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <MOUNT_POINT>  Mount point of the container
//!   <PATH>         Path of the container
//!   <NAMESPACE>    Name of the container
//!   <ID>           ID of the container (max 8 characters)
//! ```
//! <u> Options: </u>
//! ```bash
//! -h, --help  Print help
//! ```
//!
//! ### Close
//! This is a subcommand to close an existing Container.
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli close <MOUNT_POINT> <NAMESPACE>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <MOUNT_POINT>  Mount point of the container
//!   <NAMESPACE>    Name of the container
//! ```
//! <u> Options: </u>
//! ```bash
//! -h, --help  Print help
//! ```
//! ### Export
//! This is a subcommand to export an existing Container to transfer it to a different system.
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli export <PATH> <NAMESPACE> <ID> <SECRET>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <PATH>       Path of the container
//!   <NAMESPACE>  Name of the container
//!   <ID>         ID of the container (max 8 characters)
//!   <SECRET>     Secret phrase of the container (needed for importing the container)
//! ```bash
//! <u> Options: </u>
//! ```bash
//! -h, --help  Print help
//! ```
//! ### Import
//! This is a subcommand to import an existing Container that was exported on another system.
//!
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli import <PATH> <NAMESPACE> <ID> <SECRET>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <PATH>       Path of the container
//!   <NAMESPACE>  Name of the container
//!   <ID>         ID of the container (max 8 characters)
//!   <SECRET>     Secret phrase of the container
//! ```
//! <u> Options: </u>
//! ```bash
//! -h, --help  Print help
//! ```
//!
//! ### AddAutoOpen
//! This is a subcommand
//! for adding an existing Container to the AutoOpen file
//! so that it gets automatically opened on startup.
//!
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli add-auto-open <MOUNT_POINT> <PATH> <NAMESPACE> <ID>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <MOUNT_POINT>  Mount point of the container
//!   <PATH>         Path of the container
//!   <NAMESPACE>    Name of the container
//!   <ID>           ID of the container (max 8 characters)
//! ```
//! <u> Options: </u>
//! ```bash
//! -h, --help  Print help
//! ```
//! ### RemoveAutoOpen
//! This is a subcommand
//! for removing an existing Container from the AutoOpen file
//! so that it will no longer be automatically opened on startup.
//!
//! <u> Usage: </u>
//! ```bash
//! secure_container_cli remove-auto-open <MOUNT_POINT> <PATH> <NAMESPACE> <ID>
//! ```
//! <u> Arguments: </u>
//! ```bash
//!   <MOUNT_POINT>  Mount point of the container
//!   <PATH>         Path of the container
//!   <NAMESPACE>    Name of the container
//!   <ID>           ID of the container (max 8 characters)
//! ```
//! <u> Options: </u>
//! ```bash
//! -h, --help  Print help
//! ```
//!
//!
//! # Exit codes
//! The CLI returns the following exit codes:
//! ```bash
//! 0  - OK
//! 1  - The given size of the Container is too small. It must be at least 16MB.
//! 2  - The given mountpoint does not exist.
//! 3  - The given path to the Container file dose not.
//! 4  - The given Namespace for the Container is not valid. The namespace must be a string containing only ascii characters and no '|'.
//! 5  - The given ID for the Container is not valid. The ID must be a string containing only ascii characters and no '|'.
//! 6  - A container with the given name already exists and is in use.
//! 7  - An error occurred while reading the stdout of a command.
//! 8  - An error occurred while unmounting the Container.
//! 9  - An error occurred while mounting the Container.
//! 10 - An error occurred while creating a file system for the Container.
//! 11 - An error occurred while checking the logical volumes of the system.
//! 12 - An error occurred while using cryptsetup.
//! 13 - An error occurred while reading from stdin.
//! 14 - An error occurred while creating a file.
//! 15 - An error occurred while writing to a file.
//! 16 - An error occurred while deriving the key for the Container with libuta.
//! 17 - An error occurred while reading from a file.
//! 18 - An error occurred while opening a file.
//! 19 - An integrity error occurred in the Container that was opened.
//! 20 - The given Container is already mounted.
//! 21 - The given Container is already open.
//! 22 - A container with the given name already exists.
//! 23 - A file with the given name already exists at the given path.
//! 24 - The given secret is not valid. The secret must be a string containing only ascii characters and not be empty.
//! 25 - The given path is not a LUKS container.
//! 26 - The given path is not valid.
//! 27 - The given path is not a LUKS device.
//! 28 - An unknown error occurred.
//! ```
//!



mod args;
use args::{SecureContainerCli, SubCommand};
use clap::Parser;
use signal_hook::low_level::exit;
use secure_container_lib::*;


/// Import the generated gRPC code.
pub mod secure_container_service {
    tonic::include_proto!("secure_container_service");
}

/// Main function of the CLI that handles the connection to the gRPC server (demon) and the different subcommands.
/// # Return
/// 'Result<(), String>' - A result that is OK(()) if the function was successful and an error message if an error occurred.


fn main() -> Result<(), String> {
    let args = SecureContainerCli::parse();
    match args.subcmd {
        SubCommand::Create(create_args) => {
            match create_container_sync(
                create_args.size,
                create_args.mount_point,
                create_args.path,
                create_args.namespace,
                create_args.id,
                create_args.auto_open,
            ){
                Ok(_) => {
                    println!("Container created successfully.");
                }
                Err(err) => {
                    eprintln!("Error creating container: {}", err);
                    exit(error_to_exit_code(err));
                }
            }

        }
        SubCommand::Open(open_args) => {
            match open_container_sync(
                open_args.mount_point,
                open_args.path,
                open_args.namespace,
                open_args.id,
            ){
                Ok(_) => {
                    println!("Container opened successfully.");
                }
                Err(err) => {
                    eprintln!("Error opening container: {}", err);
                    exit(error_to_exit_code(err));
                }
            }
        }
        SubCommand::Close(close_args) => {
            match close_container_sync(
                close_args.mount_point,
                close_args.namespace,
            ){
                Ok(_) => {
                    println!("Container closed successfully.");
                }
                Err(err) => {
                    eprintln!("Error closing container: {}", err);
                    exit(error_to_exit_code(err));
                }
            }

        }
        SubCommand::Export(export_args) => {
            match export_container_sync(
                export_args.path,
                export_args.namespace,
                export_args.id,
                export_args.secret,
            ){
                Ok(_) => {
                    println!("Container exported successfully.");
                }
                Err(err) => {
                    eprintln!("Error exporting container: {}", err);
                    exit(error_to_exit_code(err));
                }
            }

        }
        SubCommand::Import(import_args) => {
            match import_container_sync(
                import_args.path,
                import_args.namespace,
                import_args.id,
                import_args.secret,
            ){
                Ok(_) => {
                    println!("Container imported successfully.");
                }
                Err(err) => {
                    eprintln!("Error importing container: {}", err);
                    exit(error_to_exit_code(err));
                }
            }

        }
        SubCommand::AddAutoOpen(auto_open_args) => {
            match add_container_to_auto_open_sync(
                auto_open_args.mount_point,
                auto_open_args.path,
                auto_open_args.namespace,
                auto_open_args.id,
            ){
                Ok(_) => {
                    println!("Container added to AutoOpen successfully.");
                }
                Err(err) => {
                    eprintln!("Error adding container to AutoOpen: {}", err);
                    exit(error_to_exit_code(err));
                }
            }

        }
        SubCommand::RemoveAutoOpen(auto_open_args) => {
            match remove_container_from_auto_open_sync(
                auto_open_args.mount_point,
                auto_open_args.path,
                auto_open_args.namespace,
                auto_open_args.id,
            ){
                Ok(_) => {
                    println!("Container removed from AutoOpen successfully.");
                }
                Err(err) => {
                    eprintln!("Error removing container from AutoOpen: {}", err);
                    exit(error_to_exit_code(err));
                }
            }

        }
    }

    Ok(())
}

/// Function that covert Rust error into exit codes.
/// # Arguments
/// * `err` - A string that represents the error.
/// # Returns
/// 'i32' - An exit code that represents the given error.
/// # Example
/// ```
/// let exit_code = error_to_exit_code("Size of container to small".to_string());
/// assert_eq!(exit_code, 1);
/// ```
fn error_to_exit_code(err: String) -> i32 {
    match err.as_str() {
        "Size of container to small" => 1,
        "Mountpoint wrong" => 2,
        "Not valid path" => 3,
        "Not valid namespace" => 4,
        "Not valid id" => 5,
        "Lsblk error" => 6,
        "Reading stdout error" => 7,
        "Umount error" => 8,
        "Mount error" => 9,
        "Mkfs error" => 10,
        "Ls error" => 11,
        "Cryptsetup error" => 12,
        "Stdin error" => 13,
        "File creation error" => 14,
        "File write error" => 15,
        "Libuta derive key error" => 16,
        "File read error" => 17,
        "File open error" => 18,
        "Integrity error" => 19,
        "Container mounted" => 20,
        "Container open" => 21,
        "Container with that name already exists" => 22,
        "File already exists" => 23,
        "Secret not valid" => 24,
        "Path is not a luks container" => 25,
        "Path not valid" => 26,
        "Path is not a luks divice" => 27,
        "OK" => 0,
        _ => 28,
    }
}

#[test]
fn test_error_to_exitcode() {
    assert_eq!(
        error_to_exit_code("Size of container to small".to_string()),
        1
    );
    assert_eq!(error_to_exit_code("Mountpoint wrong".to_string()), 2);
    assert_eq!(error_to_exit_code("Not valid path".to_string()), 3);
    assert_eq!(error_to_exit_code("Not valid namespace".to_string()), 4);
    assert_eq!(error_to_exit_code("Not valid id".to_string()), 5);
    assert_eq!(error_to_exit_code("Lsblk error".to_string()), 6);
    assert_eq!(error_to_exit_code("Reading stdout error".to_string()), 7);
    assert_eq!(error_to_exit_code("Umount error".to_string()), 8);
    assert_eq!(error_to_exit_code("Mount error".to_string()), 9);
    assert_eq!(error_to_exit_code("Mkfs error".to_string()), 10);
    assert_eq!(error_to_exit_code("Ls error".to_string()), 11);
    assert_eq!(error_to_exit_code("Cryptsetup error".to_string()), 12);
    assert_eq!(error_to_exit_code("Stdin error".to_string()), 13);
    assert_eq!(error_to_exit_code("File creation error".to_string()), 14);
    assert_eq!(error_to_exit_code("File write error".to_string()), 15);
    assert_eq!(
        error_to_exit_code("Libuta derive key error".to_string()),
        16
    );
    assert_eq!(error_to_exit_code("File read error".to_string()), 17);
    assert_eq!(error_to_exit_code("File open error".to_string()), 18);
    assert_eq!(error_to_exit_code("Integrity error".to_string()), 19);
    assert_eq!(error_to_exit_code("Container mounted".to_string()), 20);
    assert_eq!(error_to_exit_code("Container open".to_string()), 21);
    assert_eq!(
        error_to_exit_code("Container with that name already exists".to_string()),
        22
    );
    assert_eq!(error_to_exit_code("File already exists".to_string()), 23);
    assert_eq!(error_to_exit_code("Secret not valid".to_string()), 24);
    assert_eq!(
        error_to_exit_code("Path is not a luks container".to_string()),
        25
    );
    assert_eq!(error_to_exit_code("Path not valid".to_string()), 26);
    assert_eq!(
        error_to_exit_code("Path is not a luks divice".to_string()),
        27
    );
    assert_eq!(error_to_exit_code("OK".to_string()), 0);
    assert_eq!(error_to_exit_code("Not valid".to_string()), 28);
}
