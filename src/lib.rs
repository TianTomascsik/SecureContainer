//!
//! This library provides a set of functions to interact with the secure container service.
//!
//! ## Usage
//! This library can be used to communicate with the secure container daemon.
//!
//! ## Error
//! This library returns a string with the error message. This error message is given by the secure container daemon.
//!
//!         "Size of container to small",
//!         "Mountpoint wrong",
//!         "Not valid path",
//!         "Not valid namespace",
//!         "Not valid id",
//!         "Lsblk error",
//!         "Reading stdout error",
//!         "Umount error",
//!         "Mount error",
//!         "Mkfs error",
//!         "Ls error",
//!         "Cryptsetup error",
//!         "Stdin error",
//!         "File creation error",
//!         "File write error",
//!         "Libuta derive key error",
//!         "File read error",
//!         "File open error",
//!         "Integrity error",
//!         "Container mounted",
//!         "Container open",
//!         "Container with that name already exists",
//!         "File already exists",
//!         "Secret not valid",
//!         "Path is not a luks container",
//!         "Path not valid",
//!         "Path is not a luks device",
//!         "OK"
use tonic::{transport::{Channel}, Request, Status};
use secure_container_service::container_client::ContainerClient;
use secure_container_service::{
    AddToAutoOpenRequest, CloseContainerRequest, CreateContainerRequest, ExportContainerRequest,
    ImportContainerRequest, OpenContainerRequest, RemoveFromAutoOpenRequest,
};

pub mod secure_container_service {
    tonic::include_proto!("secure_container_service");
}

    /// Server URL
    const SERVER_URL: &'static str = "http://[::1]:50051";

    /// Synchronous wrapper for creating a container
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
    /// * `Ok(())` if the container was created successfully.
    /// * `Err(String)` with the error message if the container was not created successfully.
    /// # Examples
    /// For example usage see cli.rs.
    pub fn create_container_sync(size: i32, mount_point: String, path: String, namespace: String, id: String, auto_open: bool) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            create_container(size, mount_point, path, namespace, id, auto_open).await
        })
    }

    /// Synchronous wrapper for opening a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// # Returns
    /// * `Ok(())` if the container was opened successfully.
    /// * `Err(String)` with the error message if the container was not opened successfully.
    /// # Examples
    /// For example usage see cli.rs.
    pub fn open_container_sync(mount_point: String, path: String, namespace: String, id: String) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            open_container(mount_point, path, namespace, id).await
        })
    }

    /// Synchronous wrapper for closing a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `namespace` - The name of the container.
    /// # Returns
    /// * `Ok(())` if the container was closed successfully.
    /// * `Err(String)` with the error message if the container was not closed successfully.
    /// # Examples
    /// For example usage see cli.rs.
    pub fn close_container_sync(mount_point: String, namespace: String) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            close_container(mount_point, namespace).await
        })
    }

    /// Synchronous wrapper for exporting a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// * `secret` - The secret for the container (is needed when container is imported).
    /// # Returns
    /// * `Ok(())` if the container was exported successfully.
    /// * `Err(String)` with the error message if the container was not exported successfully.
    /// # Examples
    /// For example usage see cli.rs.
    pub fn export_container_sync(path: String, namespace: String, id: String, secret: String) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            export_container(path, namespace, id, secret).await
        })
    }

    /// Synchronous wrapper for importing a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// * `secret` - The secret for the container (is needed when container is imported).
    /// # Returns
    /// * `Ok(())` if the container was imported successfully.
    /// * `Err(String)` with the error message if the container was not imported successfully.
    /// # Examples
    /// For example usage see cli.rs.
    pub fn import_container_sync(path: String, namespace: String, id: String, secret: String) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            import_container(path, namespace, id, secret).await
        })
    }

    /// Synchronous wrapper for adding container to auto open file
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// # Returns
    /// * `Ok(())` if the container was added to auto open file successfully.
    /// * `Err(String)` with the error message if the container was not added to auto open file successfully.
    /// # Examples
    /// For example usage see cli.rs.

    pub fn add_container_to_auto_open_sync(mount_point: String, path: String, namespace: String, id: String) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            add_container_to_auto_open(mount_point, path, namespace, id).await
        })
    }

    /// Synchronous wrapper for removing container from auto open file
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// # Returns
    /// * `Ok(())` if the container was removed from auto open file successfully.
    /// * `Err(String)` with the error message if the container was not removed from auto open file successfully.
    /// # Examples
    /// For example usage see cli.rs.
    pub fn remove_container_from_auto_open_sync(mount_point: String, path: String, namespace: String, id: String) -> Result<(), String> {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            remove_container_from_auto_open(mount_point, path, namespace, id).await
        })
    }

    /// Asynchronously creates a container
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
    /// * `Ok(())` if the container was created successfully.
    /// * `Err(String)` with the error message if the container was not created successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn create_container(size: i32, mount_point: String, path: String, namespace: String, id: String, auto_open: bool) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(CreateContainerRequest {
            size,
            mount_point,
            path,
            namespace,
            id,
            auto_open,
        });

        let response = client.create_container(request).await
            .map_err(|err| format!("Error creating container: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)
        }
    }

    /// Asynchronously opens a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// # Returns
    /// * `Ok(())` if the container was opened successfully.
    /// * `Err(String)` with the error message if the container was not opened successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn open_container(mount_point: String, path: String, namespace: String, id: String) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(OpenContainerRequest {
            mount_point,
            path,
            namespace,
            id,
        });

        let response = client.open_container(request).await
            .map_err(|err| format!("Error opening container: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)
        }
    }

    /// Asynchronously closes a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `namespace` - The name of the container.
    /// # Returns
    /// * `Ok(())` if the container was closed successfully.
    /// * `Err(String)` with the error message if the container was not closed successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn close_container(mount_point: String, namespace: String) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(CloseContainerRequest {
            mount_point,
            namespace,
        });

        let response = client.close_container(request).await
            .map_err(|err| format!("Error closing container: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)
        }
    }

    /// Asynchronously exports a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// * `secret` - The secret for the container (is needed when container is imported).
    /// # Returns
    /// * `Ok(())` if the container was exported successfully.
    /// * `Err(String)` with the error message if the container was not exported successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn export_container(path: String, namespace: String, id: String, secret: String) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(ExportContainerRequest {
            path,
            namespace,
            id,
            secret,
        });

        let response = client.export_container(request).await
            .map_err(|err| format!("Error exporting container: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)
        }
    }

    /// Asynchronously imports a container
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// * `secret` - The secret for the container (is needed when container is imported).
    /// # Returns
    /// * `Ok(())` if the container was imported successfully.
    /// * `Err(String)` with the error message if the container was not imported successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn import_container(path: String, namespace: String, id: String, secret: String) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(ImportContainerRequest {
            path,
            namespace,
            id,
            secret,
        });

        let response = client.import_container(request).await
            .map_err(|err| format!("Error importing container: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)
        }
    }

    /// Asynchronously Add container to auto open file
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// # Returns
    /// * `Ok(())` if the container was added to auto open file successfully.
    /// * `Err(String)` with the error message if the container was not added to auto open file successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn add_container_to_auto_open(mount_point: String, path: String, namespace: String, id: String) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(AddToAutoOpenRequest {
            mount_point,
            path,
            namespace,
            id,
        });

        let response = client.add_to_auto_open(request).await
            .map_err(|err| format!("Error adding container to auto open: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)        }
    }

    /// Asynchronously Remove container from auto open file
    /// # Arguments
    /// * `mount_point` - The path to the mount point (must already exist).
    /// * `path` - The path to the container.
    /// * `namespace` - The name of the container.
    /// * `id` - The id of the container.
    /// # Returns
    /// * `Ok(())` if the container was removed from auto open file successfully.
    /// * `Err(String)` with the error message if the container was not removed from auto open file successfully.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn remove_container_from_auto_open(mount_point: String, path: String, namespace: String, id: String) -> Result<(), String> {
        let mut client = connect().await.map_err(|e| e.to_string())?;

        let request = Request::new(RemoveFromAutoOpenRequest {
            mount_point,
            path,
            namespace,
            id,
        });

        let response = client.remove_from_auto_open(request).await
            .map_err(|err| format!("Error removing container from auto open: {}", err))?;

        let inner = response.into_inner();
        if inner.status {
            Ok(())
        } else {
            Err(inner.error)
        }
    }

    /// Asynchronously connects to the gRPC server using the server URL.
    /// # Arguments
    /// * `None`
    /// # Returns
    /// * `Ok(ContainerClient<Channel>)` if the connection was successful.
    /// * `Err(Status)` with the error message if the connection was not successful.
    /// # Note
    /// This function is asynchronous and is not mend to be called directly.
    async fn connect() -> Result<ContainerClient<Channel>, Status> {
        ContainerClient::connect(SERVER_URL).await.map_err(|err| Status::new(tonic::Code::Unavailable, format!("Error connecting to server: {}", err)))
    }





