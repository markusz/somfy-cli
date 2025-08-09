use clap::Parser;
use log::debug;
use sdk::api_client::ApiClient;

#[derive(Parser, Debug)]
#[command(name = "somfy-cli", version, about = "Somfy CLI")]
struct Cli {
    /// Somfy API key (or set SOMFY_API_KEY)
    #[arg(long, env = "SOMFY_API_KEY")]
    api_key: String,

    /// Gateway PIN (or set SOMFY_GATEWAY_PIN)
    #[arg(long, env = "SOMFY_GATEWAY_PIN")]
    gateway_pin: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Cli::parse();

    let api_client = ApiClient::from(&args.gateway_pin, &args.api_key);
    let res_version = api_client.get_version().await;
    let res_gateways = api_client.get_gateways().await;
    let res_devices = api_client.get_devices().await;
    debug!("{res_version:?}");
    debug!("{res_gateways:?}");
    debug!("{res_devices:?}");

    Ok(())
}