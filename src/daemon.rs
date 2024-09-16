//! # daemon
//! This is the daemon that will be running on the system.
//! It functions as a gRPC server that listens to port 50051 for requests.
//! On startup, the daemon checks if any containers should be automatically opened and opens them.
//! The daemon is able to create, open, close, export, import containers and add or remove them from the autoOpen file.
//! The daemon also shuts down gracefully when a SIGINT or SIGTERM signal is received.
//! When the daemon shuts down, it checks if containers were opened by the autoOpen process and trys to close them.
//!
//! ## Usage
//! Start the daemon by running the following command (needs to be run as root):
//! ```bash
//! secure_container_daemon
//! ```
//! The daemon is now running and listening for requests.
//! The daemon can be stopped by sending a SIGINT or SIGTERM signal.
//!
//! ## Error
//! If the daemon is not able to start or an error occurs, the generated error message will be printed.
//!
//!
mod cryptsetup_wrapper;
use cryptsetup_wrapper::{
    close_container, create_container, export_container, import_container, open_container,
};
mod utilities;
use utilities::{auto_close, auto_open};

mod file_system_operations;
use file_system_operations::check_if_file_exists;

mod file_io_operations;
use file_io_operations::{add_to_auto_open, remove_auto_open};
mod error_handling;

use file_io_operations::PATH_TO_AUTO_OPEN;

use ctrlc;

use tonic::{transport::Server, Request, Response, Status};

use secure_container_service::container_server::{Container, ContainerServer};

use crate::error_handling::SecureContainerErr;
use secure_container_service::{
    CreateContainerRequest, OpenContainerRequest, SecureContainerResponse,
};

pub mod secure_container_service {
    tonic::include_proto!("secure_container_service");
}

#[derive(Debug, Default)]
pub struct MySecureContainer {}

/// Implementation of the Container trait for the MySecureContainer struct.
/// This implementation allows the daemon to handle the client requests and return the right responses.
#[tonic::async_trait]
impl Container for MySecureContainer {
    async fn create_container(
        &self,
        request: Request<CreateContainerRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = create_container(
            request.size,
            request.mount_point.as_str(),
            request.path.as_str(),
            request.namespace.as_str(),
            request.id.as_str(),
            request.auto_open,
        );
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }
    async fn open_container(
        &self,
        request: Request<OpenContainerRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = open_container(
            request.mount_point.as_str(),
            request.path.as_str(),
            request.namespace.as_str(),
            request.id.as_str(),
        );
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }
    async fn close_container(
        &self,
        request: Request<secure_container_service::CloseContainerRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = close_container(request.mount_point.as_str(), request.namespace.as_str());
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }
    async fn export_container(
        &self,
        request: Request<secure_container_service::ExportContainerRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = export_container(
            request.path.as_str(),
            request.namespace.as_str(),
            request.id.as_str(),
            request.secret.as_str(),
        );
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }
    async fn import_container(
        &self,
        request: Request<secure_container_service::ImportContainerRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = import_container(
            request.path.as_str(),
            request.namespace.as_str(),
            request.id.as_str(),
            request.secret.as_str(),
        );
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }
    async fn add_to_auto_open(
        &self,
        request: Request<secure_container_service::AddToAutoOpenRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = add_to_auto_open(
            request.mount_point.as_str(),
            request.path.as_str(),
            request.namespace.as_str(),
            request.id.as_str(),
        );
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }

    async fn remove_from_auto_open(
        &self,
        request: Request<secure_container_service::RemoveFromAutoOpenRequest>,
    ) -> Result<Response<SecureContainerResponse>, Status> {
        let request = request.into_inner();

        let result = remove_auto_open(
            request.mount_point.as_str(),
            request.path.as_str(),
            request.namespace.as_str(),
            request.id.as_str(),
        );
        let binding = result.err().unwrap_or(SecureContainerErr::OK).to_string();
        let err = binding.as_str();
        let mut status = false;
        if err == "OK" {
            status = true;
        }
        let response = secure_container_service::SecureContainerResponse {
            status,
            error: err.into(),
        };

        Ok(Response::new(response))
    }
}

/// This is the main function of the daemon.
/// It starts the daemon and listens to port 50051 for requests.
/// It also handles the SIGINT and SIGTERM signals to initialize the graceful shutdown.
/// # Return
/// `Result<(), Box<dyn std::error::Error>>`: Returns an error if the daemon is not able to start.
///
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let secure_container = MySecureContainer::default();
    match auto_open() {
        Ok(_) => (),
        Err(err) => println!("Error while Auto Open: {:?}", err),
    };

    //Channel to signal shutdown
    let (tx, _rx) = std::sync::mpsc::channel();

    //Signal handling
    let tx_clone = tx.clone();
    ctrlc::set_handler(move || {
        graceful_shutdown();
        tx_clone.send(()).unwrap();
    })
    .expect("Error setting Ctrl-C handler");

    match Server::builder()
        .add_service(ContainerServer::new(secure_container))
        .serve(addr)
        .await
    {
        Ok(_) => (),
        Err(err) => println!("{:?}", err),
    };
    Ok(())
}

/// This function is called when a SIGINT or SIGTERM signal is received.
/// This function checks if a container was open by the autoOpen process and tries to close it.
/// When the containers are closed successfully, the daemon exits with code 0.
fn graceful_shutdown() {
    let bind: &str;
    unsafe {
        bind = PATH_TO_AUTO_OPEN;
    }
    if check_if_file_exists(bind) {
        match auto_close() {
            Ok(_) => (),
            Err(err) => println!("{:?}", err),
        };
    }
    std::process::exit(0);
}
