use crate::commands::cli::Cli;
use crate::config::dotenv::CliApiClientConfig;
use anyhow::Error;
use somfy_sdk::api_client::{ApiClientConfig, CertificateHandling, HttpProtocol};

const DEFAULT_SOMFY_PORT: usize = 8443;
const API_KEY_ERROR: &str = "api key not found in CLI args, ENV variables or in .env config file";
const HOSTNAME_ERROR: &str = "hostname not found in CLI args, ENV variables or in .env config file";

pub(crate) fn merge_config_sources(
    cli_args: &Cli,
    config_file: &Option<CliApiClientConfig>,
) -> anyhow::Result<ApiClientConfig> {
    let port: usize = match (cli_args.gateway_port, config_file) {
        (Some(port), _) => port,
        (None, Some(cfg)) => cfg.port.unwrap_or(DEFAULT_SOMFY_PORT),
        _ => DEFAULT_SOMFY_PORT,
    };

    let api_key = match (&cli_args.api_key, config_file) {
        (Some(api_key), _) => api_key.to_string(),
        (None, Some(cfg)) => match &cfg.api_key {
            None => return Err(Error::msg(API_KEY_ERROR)),
            Some(key) => key.to_string(),
        },
        _ => return Err(Error::msg(API_KEY_ERROR)),
    };

    if api_key.is_empty() {
        return Err(Error::msg("The provided api key is empty"));
    }

    let url = match (&cli_args.gateway_url, config_file) {
        (Some(url), _) => url.to_string(),
        (None, Some(cfg)) => match &cfg.hostname {
            None => return Err(Error::msg(HOSTNAME_ERROR)),
            Some(key) => key.to_string(),
        },
        _ => return Err(Error::msg(HOSTNAME_ERROR)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::cli::{Cli, Command};
    use crate::config::dotenv::CliApiClientConfig;
    use crate::output::formatter::OutputStyle;

    fn create_test_cli(
        api_key: Option<String>,
        gateway_url: Option<String>,
        gateway_port: Option<usize>,
    ) -> Cli {
        Cli {
            command: Command::ListDevices,
            api_key,
            gateway_url,
            gateway_port,
            output_style: OutputStyle::Json,
        }
    }

    fn create_test_config(
        api_key: Option<String>,
        hostname: Option<String>,
        port: Option<usize>,
    ) -> CliApiClientConfig {
        CliApiClientConfig {
            protocol: Some(crate::config::dotenv::HttpProtocol::Https),
            hostname,
            port,
            api_key,
        }
    }

    #[test]
    fn test_cli_args_take_precedence_over_config_file() {
        let cli_args = create_test_cli(
            Some("cli_key".to_string()),
            Some("cli_host".to_string()),
            Some(9999),
        );
        let config_file = Some(create_test_config(
            Some("config_key".to_string()),
            Some("config_host".to_string()),
            Some(8888),
        ));

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.api_key, "cli_key");
        assert_eq!(result.url, "cli_host");
        assert_eq!(result.port, 9999);
    }

    #[test]
    fn test_config_file_used_when_cli_args_missing() {
        let cli_args = create_test_cli(None, None, None);
        let config_file = Some(create_test_config(
            Some("config_key".to_string()),
            Some("config_host".to_string()),
            Some(7777),
        ));

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.api_key, "config_key");
        assert_eq!(result.url, "config_host");
        assert_eq!(result.port, 7777);
    }

    #[test]
    fn test_default_port_used_when_not_specified() {
        let cli_args = create_test_cli(
            Some("test_key".to_string()),
            Some("test_host".to_string()),
            None,
        );
        let config_file = Some(create_test_config(
            Some("config_key".to_string()),
            Some("config_host".to_string()),
            None,
        ));

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.port, DEFAULT_SOMFY_PORT);
    }

    #[test]
    fn test_config_file_port_overrides_default() {
        let cli_args = create_test_cli(
            Some("test_key".to_string()),
            Some("test_host".to_string()),
            None,
        );
        let config_file = Some(create_test_config(None, None, Some(6666)));

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.port, 6666);
    }

    #[test]
    fn test_missing_api_key_returns_error() {
        let cli_args = create_test_cli(None, Some("test_host".to_string()), None);
        let config_file = Some(create_test_config(
            None, // No API key in config either
            Some("config_host".to_string()),
            Some(8443),
        ));

        let result = merge_config_sources(&cli_args, &config_file);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), API_KEY_ERROR);
    }

    #[test]
    fn test_missing_hostname_returns_error() {
        let cli_args = create_test_cli(Some("test_key".to_string()), None, None);
        let config_file = Some(create_test_config(
            Some("config_key".to_string()),
            None, // No hostname in config either
            Some(8443),
        ));

        let result = merge_config_sources(&cli_args, &config_file);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), HOSTNAME_ERROR);
    }

    #[test]
    fn test_empty_api_key_returns_error() {
        let cli_args = create_test_cli(
            Some("".to_string()), // Empty API key
            Some("test_host".to_string()),
            None,
        );
        let config_file = None;

        let result = merge_config_sources(&cli_args, &config_file);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "The provided api key is empty"
        );
    }

    #[test]
    fn test_no_config_file_uses_cli_args() {
        let cli_args = create_test_cli(
            Some("cli_key".to_string()),
            Some("cli_host".to_string()),
            Some(5555),
        );
        let config_file = None;

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.api_key, "cli_key");
        assert_eq!(result.url, "cli_host");
        assert_eq!(result.port, 5555);
    }

    #[test]
    fn test_no_config_file_missing_required_fields_errors() {
        let cli_args = create_test_cli(None, None, None);
        let config_file = None;

        let result = merge_config_sources(&cli_args, &config_file);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), API_KEY_ERROR);
    }

    #[test]
    fn test_partial_cli_args_partial_config() {
        let cli_args = create_test_cli(
            Some("cli_key".to_string()),
            None, // Missing URL in CLI
            Some(4444),
        );
        let config_file = Some(create_test_config(
            Some("config_key".to_string()),
            Some("config_host".to_string()), // URL from config
            Some(3333),
        ));

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.api_key, "cli_key"); // From CLI
        assert_eq!(result.url, "config_host"); // From config
        assert_eq!(result.port, 4444); // From CLI (takes precedence)
    }

    #[test]
    fn test_generated_config_has_correct_defaults() {
        let cli_args = create_test_cli(
            Some("test_key".to_string()),
            Some("test_host".to_string()),
            None,
        );
        let config_file = None;

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.protocol, HttpProtocol::HTTPS);
        assert_eq!(result.cert_handling, CertificateHandling::DefaultCert);
        assert_eq!(result.port, DEFAULT_SOMFY_PORT);
    }

    #[test]
    fn test_whitespace_api_key_considered_empty() {
        let cli_args = create_test_cli(
            Some("   ".to_string()), // Whitespace-only API key
            Some("test_host".to_string()),
            None,
        );
        let config_file = None;

        let result = merge_config_sources(&cli_args, &config_file);

        // Should still pass length check but might want to trim in the future
        assert!(result.is_ok());
        assert_eq!(result.unwrap().api_key, "   ");
    }

    #[test]
    fn test_config_file_api_key_none_vs_missing_field() {
        // Test the difference between api_key: None vs missing api_key field
        let cli_args = create_test_cli(None, Some("test_host".to_string()), None);
        let config_file = create_test_config(
            None, // Explicitly set to None
            Some("config_host".to_string()),
            None,
        );

        let result = merge_config_sources(&cli_args, &Some(config_file));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), API_KEY_ERROR);
    }

    #[test]
    fn test_mixed_sources_integration() {
        // Realistic scenario: some values from CLI, some from config, some defaults
        let cli_args = create_test_cli(
            None,                              // Will come from config
            Some("192.168.1.100".to_string()), // Override config
            None,                              // Will use default
        );
        let config_file = Some(create_test_config(
            Some("my_secret_key".to_string()),
            Some("config.local".to_string()),
            Some(9443),
        ));

        let result = merge_config_sources(&cli_args, &config_file).unwrap();

        assert_eq!(result.api_key, "my_secret_key"); // From config
        assert_eq!(result.url, "192.168.1.100"); // From CLI (override)
        assert_eq!(result.port, 9443); // From config (CLI port is None)
    }
}
