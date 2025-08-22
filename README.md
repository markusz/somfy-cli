# Somfy CLI

A command-line interface for controlling Somfy smart home devices via the TaHoma Local API.

## Overview

The Somfy CLI provides comprehensive control over Somfy smart home devices, allowing you to discover devices, control device states, manage aliases, and monitor device events directly from the command line.

## Installation

### From Source

```bash
git clone https://github.com/markusz/somfy-cli.git
cd somfy-cli
cargo build --release
```

The compiled binary will be available at `target/release/somfy`.

### Using Cargo

```bash
cargo install --path .
```

## Authentication

The CLI supports multiple authentication methods, with the following order of precedence:

### 1. Command Line Parameters (Highest Priority)

```bash
somfy --api-key YOUR_API_KEY --gateway-url gateway.local.ip --gateway-port 8443 ls
```

### 2. Environment Variables

```bash
export SOMFY_API_KEY=your_api_key_here
export SOMFY_GATEWAY_HOSTNAME=192.168.1.100
export SOMFY_GATEWAY_PORT=8443
somfy ls
```

### 3. Configuration File (.env.json)

Create a configuration file at `~/.config/somfy-cli/env.json` (or `%APPDATA%\somfy-cli\env.json` on Windows):

```json
{
  "protocol": "Https",
  "hostname": "192.168.1.100",
  "port": 8443,
  "api_key": "your_api_key_here"
}
```

## Commands

### Device Control

#### Open Device
Completely opens a device (blinds, shutters, etc.):
```bash
somfy open <device_url_or_alias>
```

#### Close Device
Completely closes a device:
```bash
somfy close <device_url_or_alias>
```

#### Set Position
Moves a device to a specific position (0-100%):
```bash
somfy position <device_url_or_alias> <percentage>
```

### Device Information

#### List Devices
Lists all available devices:
```bash
somfy ls
```

#### Current Executions
Shows all currently running device executions:
```bash
somfy current-execs
```

#### Listen for Events
Listens for real-time device events:
```bash
somfy listen
```

### Alias Management

Create and manage aliases for device URLs to simplify commands:

#### Add Alias
```bash
somfy alias add <alias_name> <device_url>
somfy alias add --overwrite <alias_name> <device_url>  # Overwrite existing alias
```

#### Remove Alias
```bash
somfy alias rm <alias_name>
```

#### List Aliases
```bash
somfy alias ls
```

#### Using Aliases
Once created, aliases can be used in place of device URLs:
```bash
somfy alias add living-room io://1234-5678-9012/device1
somfy open living-room  # Instead of: somfy open io://1234-5678-9012/device1
```

## Configurable Output Formats

The CLI supports two output formats that can be configured per-command:

### JSON Format (Default)
```bash
somfy ls --output-style json
# or
somfy -S json ls
somfy ls # Omitting the params defaults to JSON
```

Example JSON output:
```json
[
  {
    "label": "Living Room Blinds",
    "device_url": "io://1234-5678-9012/device1",
    "controllable_name": "io:StackComponent",
    "states": [
      {
        "name": "core:StatusState",
        "value": "available"
      },
      {
        "name": "core:ClosureState", 
        "value": 75
      }
    ]
  }
]
```

Output can be piped, e.g.
```
somfy ls | jq '.[].label
```


### Table Format
```bash
somfy ls --output-style table
# or  
somfy ls -S table
```

Example table output:
```
┌────────────────────┬─────────────────────────────┬─────────────────────┬─────────────┬──────────────┬─────────────┬───────────┬───────────────────┬────────────────┬────────────┐
│ Label              │ Device URL                  │ Device Type         │ Open/Close  │ Status       │ Closure (%) │ Tilt (%)  │ 'My' position (%) │ 'My' tilt (%)  │ Is Moving? │
├────────────────────┼─────────────────────────────┼─────────────────────┼─────────────┼──────────────┼─────────────┼───────────┼───────────────────┼────────────────┼────────────┤
│ Living Room Blinds │ io://1234-5678-9012/device1 │ io:StackComponent   │     closed  │    available │          75 │         0 │                50 │              0 │      false │
│ Bedroom Shutters   │ io://1234-5678-9012/device2 │ io:StackComponent   │     closed  │    available │         100 │         0 │                25 │              0 │      false │
└────────────────────┴─────────────────────────────┴─────────────────────┴─────────────┴──────────────┴─────────────┴───────────┴───────────────────┴────────────────┴────────────┘
```

## Configuration

### Required Credentials

- **API Key**: Your Somfy API authentication key
- **Gateway URL**: Your TaHoma gateway IP address or hostname  
- **Gateway Port**: Port number (typically 8443 for TaHoma Local API)

### Connection Settings

The CLI automatically configures:
- **Protocol**: HTTPS for secure connections
- **Certificate Handling**: Automatic handling of self-signed certificates
- **Timeouts**: Reasonable timeouts for API calls

## Examples

### Basic Usage
```bash
# List all devices
somfy ls

# Open living room blinds using device URL
somfy open io://1234-5678-9012/device1

# Create an alias and use it
somfy alias add living-room io://1234-5678-9012/device1
somfy close living-room

# Set blinds to 50% closed
somfy position living-room 50

# Monitor device events
somfy listen
```

### Authentication Examples
```bash
# Using environment variables
export SOMFY_API_KEY=abc123
export SOMFY_GATEWAY_HOSTNAME=192.168.1.100
somfy ls

# Using command line parameters
somfy --api-key abc123 --gateway-url 192.168.1.100 ls

# Using different output formats
somfy ls --output-style table
somfy current-execs -S json
```

## Development

### Prerequisites
- Rust 1.82.0 or later
- Access to a Somfy TaHoma gateway
- Valid API credentials

### Building
```bash
cargo build --release
```

### Running Tests
```bash
cargo test
```

### Running in Development Mode
```bash
cargo run -- ls
cargo run -- --api-key YOUR_KEY --gateway-url YOUR_GATEWAY ls
```

## Troubleshooting

### Common Issues

1. **Authentication errors**: Verify your API key and gateway URL are correct
2. **Network timeouts**: Check gateway connectivity and network settings  
3. **Permission errors**: Ensure your API key has necessary permissions
4. **Certificate errors**: The CLI handles self-signed certificates automatically

### Debug Mode

Run with detailed logging:
```bash
RUST_LOG=debug cargo run -- ls
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.