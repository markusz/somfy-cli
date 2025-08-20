use crate::utils::poller::PollerConfig;
use somfy_sdk::api_client::ApiClient;
use somfy_sdk::commands::execute_action_group::ExecuteActionGroupResponse;
use somfy_sdk::commands::get_current_executions::GetCurrentExecutionsResponse;
use somfy_sdk::commands::get_devices::GetDevicesResponse;
use somfy_sdk::commands::get_execution::GetExecutionResponse;
use somfy_sdk::commands::types::{Action, ActionGroup, Command};
use somfy_sdk::err::http::RequestError;
use std::time::SystemTime;
use tokio::time::sleep;


pub struct CommandExecutor {
    pub(crate) api_client: ApiClient,
}

pub enum OpenClose {
    Open,
    Close,
    Closure(u8),
}

impl From<OpenClose> for String {
    fn from(value: OpenClose) -> Self {
        match value {
            OpenClose::Open => "open".to_string(),
            OpenClose::Close => "close".to_string(),
            OpenClose::Closure(_) => "setClosure".to_string(),
        }
    }
}

impl CommandExecutor {
    async fn open_close(
        &self,
        device_url: String,
        state: OpenClose,
    ) -> Result<ExecuteActionGroupResponse, RequestError> {
        let params = match state {
            OpenClose::Closure(c_args) => vec![c_args.to_string()],
            _ => vec![],
        };

        let action: String = state.into();
        let action_group_label = format!("{} blinds {}", action, device_url).to_string();

        let request = ActionGroup {
            label: Some(action_group_label),
            actions: vec![Action {
                device_url,
                commands: vec![Command {
                    name: action,
                    parameters: params,
                }],
            }],
        };

        self.api_client.execute_actions(&request).await
    }

    pub(crate) async fn open(
        &self,
        device_url: String,
    ) -> Result<ExecuteActionGroupResponse, RequestError> {
        self.open_close(device_url, OpenClose::Open).await
    }

    pub(crate) async fn closure(
        &self,
        device_url: String,
        percent: u8,
    ) -> Result<ExecuteActionGroupResponse, RequestError> {
        self.open_close(device_url, OpenClose::Closure(percent))
            .await
    }

    pub(crate) async fn close(
        &self,
        device_url: String,
    ) -> Result<ExecuteActionGroupResponse, RequestError> {
        self.open_close(device_url, OpenClose::Close).await
    }

    pub(crate) async fn list_devices(&self) -> Result<GetDevicesResponse, RequestError> {
        self.api_client.get_devices().await
    }

    pub(crate) async fn get_current_executions(
        &self,
    ) -> Result<GetCurrentExecutionsResponse, RequestError> {
        self.api_client.get_current_executions().await
    }

    pub(crate) async fn get_execution(
        &self,
        id: &str,
    ) -> Result<GetExecutionResponse, RequestError> {
        self.api_client.get_execution(id).await
    }

    pub(crate) async fn listen(&self) -> Result<(), RequestError> {
        let event_listener = self.api_client.register_event_listener().await?;
        let poller_config = PollerConfig::for_event_listener();

        let now = SystemTime::now();
        sleep(poller_config.refresh_interval).await;
        while now.elapsed().map_err(|e| RequestError::Server(e.into()))? < poller_config.max_wait {
            let events = self.api_client.fetch_events(event_listener.id.as_str()).await;
            if let Ok(events) = events {
                for e in events {
                    println!("{e:?}")
                }
            }
            sleep(poller_config.refresh_interval).await;
        }

        Ok(())
    }

    /// Execution results are available asynchronously on the API.
    /// This means that calling get_execution(execId) is not guaranteed to return the full execution result
    /// The fn provides support for a configurable poller that polls that polls /exec/current/:execid for the result
    ///
    /// # Arguments
    ///
    /// * `exec_id`: The executionId to be retrieved
    /// * `poller_config`: The poller config
    ///
    /// returns: Result<ActionGroupExecution, RequestError>
    ///
    /// # Examples
    ///
    /// ```
    /// let device_url = "".to_string()
    /// let exec_resp = cmd_dispatcher.close(device_url).await?;
    /// let exec_details = cmd_dispatcher.get_execution_with_full_response(exec_resp.exec_id.as_str(), ExecutionResultPollerConfig::default()).await?;
    /// ```
    pub(crate) async fn get_execution_with_full_response(
        &self,
        exec_id: &str,
        poller_config: PollerConfig,
    ) -> Result<GetExecutionResponse, RequestError> {
        let mut res = self.api_client.get_execution(exec_id).await;
        let now = SystemTime::now();
        sleep(poller_config.refresh_interval).await;
        while res.is_err()
            && now.elapsed().map_err(|e| RequestError::Server(e.into()))? < poller_config.max_wait
        {
            res = self.api_client.get_execution(exec_id).await;
            sleep(poller_config.refresh_interval).await;
        }

        self.api_client.get_execution(exec_id).await
    }
}
