use log::debug;
use sdk::api_client::ApiClient;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let api_client = ApiClient::from("0812-2424-9999", "my_key");
    let res = api_client.get_version().await;
    debug!("{res:?}")
}
