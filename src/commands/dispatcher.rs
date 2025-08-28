use crate::commands::cli::{AliasCommands, Command};
use crate::commands::executor::CommandExecutor;
use crate::config::alias::AliasManager;
use crate::output::formatter::{print_to_console, OutputStyle};
use crate::utils::poller::PollerConfig;
use log::debug;
use somfy_sdk::api_client::ApiClient;
use somfy_sdk::commands::execute_action_group::ExecuteActionGroupResponse;
use somfy_sdk::commands::get_execution::GetExecutionResponse;

pub struct CommandDispatcher {
    cmd_executor: CommandExecutor,
}

impl CommandDispatcher {
    pub(crate) fn from(api_client: ApiClient) -> Self {
        let cmd_executor = CommandExecutor { api_client };
        Self { cmd_executor }
    }

    async fn try_poll(
        &self,
        eagr: ExecuteActionGroupResponse,
        pc: PollerConfig,
    ) -> anyhow::Result<GetExecutionResponse> {
        let res = self
            .cmd_executor
            .get_execution_with_full_response(eagr.exec_id.as_str(), pc)
            .await?;

        Ok(res)
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

                if args.no_poll {
                    print_to_console(exec_resp, style)
                } else {
                    let detailed_resp = self.try_poll(exec_resp, PollerConfig::default()).await?;
                    print_to_console(detailed_resp, style)
                };
            }
            Command::Close(args) => {
                let device_url = alias_manager.resolve_alias(&args.device_url);
                let exec_resp = self.cmd_executor.close(device_url).await?;
                if args.no_poll {
                    print_to_console(exec_resp, style)
                } else {
                    let detailed_resp = self.try_poll(exec_resp, PollerConfig::default()).await?;
                    print_to_console(detailed_resp, style)
                };
            }
            Command::ListDevices => {
                let devices_resp = self.cmd_executor.list_devices().await?;
                print_to_console(devices_resp, style);
            }
            Command::GetCurrentExecutions => {
                let execs_resp = self.cmd_executor.get_current_executions().await?;
                print_to_console(execs_resp, style);
            }
            Command::Position(args) => {
                let device_url = alias_manager.resolve_alias(&args.device_url);
                let exec_resp = self
                    .cmd_executor
                    .closure(device_url, args.percentage)
                    .await?;
                if args.no_poll {
                    print_to_console(exec_resp, style)
                } else {
                    let detailed_resp = self.try_poll(exec_resp, PollerConfig::default()).await?;
                    print_to_console(detailed_resp, style)
                };
            }
            Command::Listen => {
                debug!("Listening for events");
                let _ = self.cmd_executor.listen().await;
            }
            Command::Alias(a) => match a.alias_cmd {
                AliasCommands::Add(a) => {
                    let aliases =
                        alias_manager.add_alias(a.alias_name, a.device_url, a.overwrite)?;

                    print_to_console(aliases, style);
                }
                AliasCommands::Rm(r) => {
                    let aliases = alias_manager.delete_alias(r.alias_name)?;
                    print_to_console(aliases, style);
                }
                AliasCommands::Ls => {
                    let aliases = alias_manager.load_aliases()?;
                    print_to_console(aliases, style);
                }
            },
        }

        Ok(())
    }
}
