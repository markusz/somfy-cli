use crate::commands::cli::{AliasCommands, Command};
use crate::commands::executor::CommandExecutor;
use crate::config::alias::AliasManager;
use crate::output::formatter::{CliOutput, OutputStyle};
use crate::utils::poller::PollerConfig;
use log::debug;
use somfy_sdk::api_client::ApiClient;
use std::time::Duration;
use tokio::time::sleep;

pub struct CommandDispatcher {
    cmd_executor: CommandExecutor,
}

impl CommandDispatcher {
    pub(crate) fn from(api_client: ApiClient) -> Self {
        let cmd_executor = CommandExecutor { api_client };
        Self { cmd_executor }
    }

    pub(crate) async fn dispatch(
        &self,
        command: Command,
        style: OutputStyle,
    ) -> anyhow::Result<()> {
        let alias_manager = AliasManager::default();
        match command {
            Command::Open(args) => {
                let device_url = alias_manager.resolve_alias(&args.device_url);
                let exec_resp = self.cmd_executor.open(device_url).await?;
                let exec_details = self
                    .cmd_executor
                    .get_execution_with_full_response(
                        exec_resp.exec_id.as_str(),
                        PollerConfig::default(),
                    )
                    .await?;

                let res = exec_details.to_styled_cli_output(style)?;
                println!("{res}");
            }
            Command::Close(args) => {
                let device_url = alias_manager.resolve_alias(&args.device_url);
                let exec_resp = self.cmd_executor.close(device_url).await?;
                let exec_details = self
                    .cmd_executor
                    .get_execution_with_full_response(
                        exec_resp.exec_id.as_str(),
                        PollerConfig::default(),
                    )
                    .await?;
                let res = exec_details.to_styled_cli_output(style)?;
                println!("{res}")
            }
            Command::ListDevices => {
                let devices_resp = self.cmd_executor.list_devices().await?;
                let res = devices_resp.to_styled_cli_output(style)?;

                println!("{res}")
            }
            Command::GetCurrentExecutions => {
                let res = self.cmd_executor.get_current_executions().await;
                println!("{res:#?}")
            }
            Command::Position(p_args) => {
                let device_url = alias_manager.resolve_alias(&p_args.device_url);
                let exec_resp = self
                    .cmd_executor
                    .closure(device_url, p_args.percentage)
                    .await?;
                sleep(Duration::from_millis(2000)).await;
                let exec_details = self
                    .cmd_executor
                    .get_execution(exec_resp.exec_id.as_str())
                    .await?;
                let res = exec_details.to_styled_cli_output(style)?;
                println!("{res}");
            }
            Command::Listen => {
                debug!("Listening for events");
                let _ = self.cmd_executor.listen().await;
            }
            Command::Alias(a) => match a.alias_cmd {
                AliasCommands::Add(a) => {
                    let aliases =
                        alias_manager.add_alias(a.alias_name, a.device_url, a.overwrite)?;
                    let str = aliases.to_styled_cli_output(style)?;
                    println!("{str}");
                }
                AliasCommands::Rm(r) => {
                    let aliases = alias_manager.delete_alias(r.alias_name)?;
                    let str = aliases.to_styled_cli_output(style)?;
                    println!("{str}");
                }
                AliasCommands::Ls => {
                    let aliases = alias_manager.load_aliases()?;
                    let str = aliases.to_styled_cli_output(style)?;
                    println!("{str}");
                }
            },
        }

        Ok(())
    }
}
