use crate::commands::cli::{AliasCommands, Command};
use crate::commands::executor::CommandExecutor;
use crate::config::alias::AliasManager;
use crate::utils::poller::PollerConfig;
use log::{error, info};
use somfy_sdk::api_client::ApiClient;
use std::time::Duration;
use tabled::builder::Builder;
use tabled::settings::Style;
use tokio::time::sleep;

pub struct CommandDispatcher {
    cmd_executor: CommandExecutor,
}

impl CommandDispatcher {
    pub(crate) fn from(api_client: ApiClient) -> Self {
        let cmd_executor = CommandExecutor { api_client };
        Self { cmd_executor }
    }

    pub(crate) async fn dispatch(&self, command: Command) -> anyhow::Result<()> {
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
                println!("{exec_details:#?}");
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
                println!("{exec_details:#?}")
            }
            Command::ListDevices => {
                let res = self.cmd_executor.list_devices().await;

                let mut b = Builder::new();
                b.push_record(["name", "device_url", "vendor"]);

                match res {
                    Ok(res) => {
                        for dev in res {
                            let vendor: Option<String> = dev
                                .attributes
                                .iter()
                                .find(|a| a.name == "core:Manufacturer")
                                .map(move |d| d.value.clone().to_string());

                            b.push_record([
                                dev.label,
                                dev.device_url,
                                vendor.unwrap_or("n/a".into()),
                            ])
                        }

                        let mut table = b.build();
                        table.with(Style::sharp());
                        println!("{table}")
                    }
                    Err(e) => {
                        error!("{e}");
                    }
                }
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
                println!("{exec_details:#?}")
            }
            Command::Listen => {
                info!("Listening for events");
                let _ = self.cmd_executor.listen().await;
            }
            Command::Alias(a) => match a.alias_cmd {
                AliasCommands::Add(a) => {
                    let aliases = alias_manager.add_alias(a.alias_name, a.device_url, a.overwrite);
                    info!("{aliases:?}")
                }
                AliasCommands::Rm(r) => {
                    let aliases = alias_manager.delete_alias(r.alias_name);
                    info!("{aliases:?}")
                }
                AliasCommands::Ls => {
                    let aliases = alias_manager.load_aliases();
                    info!("{aliases:?}");
                    let builder = Builder::from(aliases?);
                    let mut table = builder.build();
                    table.with(Style::modern_rounded());
                    println!("{table}");
                }
            },
        }

        Ok(())
    }
}
