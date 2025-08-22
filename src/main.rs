pub(crate) mod commands {
    pub(crate) mod cli;
    pub(crate) mod dispatcher;
    pub(crate) mod executor;
}
pub(crate) mod utils {
    pub(crate) mod poller;
}
pub(crate) mod output {
    pub(crate) mod formatter;
}

pub(crate) mod config {
    pub(crate) mod alias;
    pub(crate) mod config_dir;
    pub(crate) mod dotenv;
    pub(crate) mod loader;
}

pub(crate) mod demos {
    pub(crate) mod demo;
}

use crate::commands::cli::Cli;
use crate::commands::dispatcher::CommandDispatcher;
use crate::config::loader::create_config_from_sources;
use clap::Parser;
use somfy_sdk::api_client::ApiClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli_args = Cli::parse();
    let config = create_config_from_sources()?;

    let api_client = ApiClient::new(config).await?;
    let cmd_dispatcher = CommandDispatcher::from(api_client);

    cmd_dispatcher
        .dispatch(cli_args.command, cli_args.output_style)
        .await?;

    Ok(())
}
