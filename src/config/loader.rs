use crate::commands::cli::Cli;
use crate::config::dotenv::load_config_file;
use anyhow::Error;
use clap::Parser;
use somfy_sdk::api_client::{ApiClientConfig, CertificateHandling, HttpProtocol};

const DEFAULT_SOMFY_PORT: usize = 8443;

pub(crate) fn create_config_from_sources() -> anyhow::Result<ApiClientConfig> {
    let config_file = load_config_file()?;
    let cli_args = Cli::parse();

    let port = match (cli_args.gateway_port, &config_file) {
        (Some(port), _) => port,
        (None, Some(cfg)) => cfg.port.unwrap_or(DEFAULT_SOMFY_PORT),
        _ => DEFAULT_SOMFY_PORT,
    };

    let api_key = match (cli_args.api_key, &config_file) {
        (Some(api_key), _) => api_key,
        (None, Some(cfg)) => cfg.api_key.clone().unwrap(),
        _ => return Err(Error::msg("API_KEY is required")),
    };

    let url = match (cli_args.gateway_url, &config_file) {
        (Some(url), _) => url,
        (None, Some(cfg)) => cfg.hostname.clone().unwrap(),
        _ => return Err(Error::msg("GATEWAY_URL is required")),
    };

    let config = ApiClientConfig {
        protocol: HttpProtocol::HTTPS,
        cert_handling: CertificateHandling::DefaultCert,
        port,
        api_key,
        url,
    };

    Ok(config)
}
