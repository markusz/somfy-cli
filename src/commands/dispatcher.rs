use crate::commands::cli::{AliasCommands, Command};
use crate::commands::executor::CommandExecutor;
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
        Self {
            cmd_executor
        }
    }

    pub(crate) async fn dispatch(&self, command: Command) -> anyhow::Result<()> {
        match command {
            Command::Open(args) => {
                let exec_resp = self.cmd_executor.open(args.device_url).await?;
                let exec_details = self.cmd_executor
                    .get_execution_with_full_response(
                        exec_resp.exec_id.as_str(),
                        PollerConfig::default(),
                    )
                    .await?;
                println!("{exec_resp:#?}");
                println!("{exec_details:#?}");
            }
            Command::Close(args) => {
                let exec_resp = self.cmd_executor.close(args.device_url).await?;
                let exec_details = self.cmd_executor
                    .get_execution_with_full_response(
                        exec_resp.exec_id.as_str(),
                        PollerConfig::default(),
                    )
                    .await?;
                println!("{exec_resp:#?}");
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
                                .and_then(move |d| Some(d.value.clone().to_string()));

                            b.push_record([dev.label, dev.device_url, vendor.unwrap_or("n/a".into())])
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
                let exec_resp = self.cmd_executor
                    .closure(p_args.device_url, p_args.percentage)
                    .await?;
                sleep(Duration::from_millis(2000)).await;
                let exec_details = self.cmd_executor
                    .get_execution(exec_resp.exec_id.as_str())
                    .await?;
                println!("{exec_resp:#?}");
                println!("{exec_details:#?}")
            }
            Command::Listen => {
                info!("Listening for events");
                let _ = self.cmd_executor.listen().await;
            }
            Command::Alias(a) => {
                match a.alias_cmd {
                    AliasCommands::Add(a) => info!("{a:?}"),
                    AliasCommands::Rm(r) => info!("{r:?}"),
                    AliasCommands::Ls => info!("List Alias"),
                }
            }
        }

        Ok(())
    }
}

