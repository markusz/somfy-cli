use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
#[command(
    name = "somfy-cli",
    version,
    about = "Somfy CLI",
    long_about = "A CLI to control Somfy devices"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
    /// Somfy API key (or set SOMFY_API_KEY)
    #[arg(long, env = "SOMFY_API_KEY")]
    pub(crate) api_key: Option<String>,

    /// Gateway PIN (or set SOMFY_GATEWAY_PIN)
    #[arg(long, env = "SOMFY_GATEWAY_HOSTNAME")]
    pub(crate) gateway_url: Option<String>,

    #[arg(long, env = "SOMFY_GATEWAY_PORT")]
    pub(crate) gateway_port: Option<usize>,
}

#[derive(Args)]
pub(crate) struct OpenArgs {
    pub(crate) device_url: String,
}

#[derive(Args, Debug)]
pub(crate) struct CloseArgs {
    pub(crate) device_url: String,
}

#[derive(Args, Debug)]
pub(crate) struct PositionArgs {
    pub(crate) device_url: String,
    pub(crate) percentage: u8,
}

#[derive(Args, Debug)]
pub(crate) struct AliasAddArgs {
    pub(crate) alias_name: String,
    pub(crate) device_url: String,
    #[arg(long, short = 'O', help = "Overwrites an existing alias")]
    pub(crate) overwrite: bool,
}

#[derive(Args, Debug)]
pub(crate) struct AliasRmArgs {
    pub(crate) alias_name: String,
}

#[derive(Subcommand, Debug)]
pub(crate) enum AliasCommands {
    Add(AliasAddArgs),
    Rm(AliasRmArgs),
    Ls,
}

#[derive(Args, Debug)]
pub(crate) struct AliasArgs {
    #[command(subcommand)]
    pub(crate) alias_cmd: AliasCommands,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    #[command(long_about = "Completely opens the device")]
    Open(OpenArgs),
    #[command(long_about = "Completely closes the device")]
    Close(CloseArgs),
    #[command(long_about = "Moves the device into the x % position")]
    Position(PositionArgs),
    #[command(name = "ls", long_about = "Lists all devices")]
    ListDevices,
    #[command(name = "current-execs", long_about = "Lists all running executions")]
    GetCurrentExecutions,
    #[command(long_about = "Listen for device events")]
    Listen,
    // Scenario,
    #[command(name = "alias", long_about = "Manage aliases for devices")]
    Alias(AliasArgs),
}
