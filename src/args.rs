/// This file contains the structr and arguments for the command line interface.
use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(
    name = "Secure Container Service",
    version = "1.0",
    author = "Tian Tomascsik"
)]
pub struct SecureContainerCli {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

/// Here are all possible subcommands for the CLI defined.
#[derive(Debug, Subcommand)]
pub enum SubCommand {
    /// Create a new container
    Create(Create),
    /// Open an existing container
    Open(Open),
    /// Close an existing container
    Close(Close),
    /// Export an existing container
    Export(Export),
    /// Import an existing container
    Import(Import),
    /// Add a container to auto open
    AddAutoOpen(AddAutoOpen),
    /// Remove a container from auto open
    RemoveAutoOpen(RemoveAutoOpen),
}

/// Definition of the subcommand 'create' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Create {
    /// Size of the container in MB
    pub size: i32,
    /// Mount point of the container
    pub mount_point: String,
    /// Path of the container
    pub path: String,
    /// Name of the container
    pub namespace: String,
    /// ID of the container
    pub id: String,
    /// Auto open the container
    #[clap(short, long)]
    pub auto_open: bool,
}

/// Definition of the subcommand 'open' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Open {
    /// Mount point of the container
    pub mount_point: String,
    /// Path of the container
    pub path: String,
    /// Name of the container
    pub namespace: String,
    /// ID of the container
    pub id: String,
}

/// Definition of the subcommand 'close' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Close {
    /// Mount point of the container
    pub mount_point: String,
    /// Name of the container
    pub namespace: String,
}

/// Definition of the subcommand 'export' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Export {
    /// Path of the container
    pub path: String,
    /// Name of the container
    pub namespace: String,
    /// ID of the container
    pub id: String,
    /// Secret phrase of the container (needed for importing the container)
    pub secret: String,
}

/// Definition of the subcommand 'import' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Import {
    /// Path of the container
    pub path: String,
    /// Name of the container
    pub namespace: String,
    /// ID of the container
    pub id: String,
    /// Secret phrase of the container
    pub secret: String,
}

/// Definition of the subcommand 'add-auto-open' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct AddAutoOpen {
    /// Mount point of the container
    pub mount_point: String,
    /// Path of the container
    pub path: String,
    /// Name of the container
    pub namespace: String,
    /// ID of the container
    pub id: String,
}

/// Definition of the subcommand 'remove-auto-open' with all its arguments.
#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct RemoveAutoOpen {
    /// Mount point of the container
    pub mount_point: String,
    /// Path of the container
    pub path: String,
    /// Name of the container
    pub namespace: String,
    /// ID of the container
    pub id: String,
}
