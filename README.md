# Somfy CLI

A command-line interface for interacting with Somfy smart home devices via the TaHoma Local API.

## Overview

The CLI provides a comprehensive demonstration of all available Somfy API endpoints, allowing you to discover devices, monitor states, handle events, and execute actions directly from the command line.

## Features

- **Complete device discovery** and state management
- **Real-time event listening** with automatic cleanup
- **Action execution** with safety guards
- **Comprehensive error handling** with user-friendly output
- **Environment variable support** for credentials
- **Colorized output** with emojis for better readability

## Installation

### From Source

```bash
git clone https://github.com/user/somfy-sdk-cli.git
cd somfy-sdk-cli
cargo build --release -p cli
```

### Using Cargo

```bash
cargo install somfy-sdk-cli
```

## Usage

### Basic Usage

```bash
# Run with credentials as arguments
cargo run -p cli -- --api-key YOUR_API_KEY --gateway-pin YOUR_GATEWAY_PIN

# Or use environment variables
export SOMFY_API_KEY=your_api_key
export SOMFY_GATEWAY_PIN=your_gateway_pin
cargo run -p cli
```

### Command Line Options

```bash
somfy-cli [OPTIONS]

Options:
    --api-key <API_KEY>          Your Somfy API key
    --gateway-pin <GATEWAY_PIN>  Your TaHoma gateway PIN/ID
    -h, --help                   Print help information
    -V, --version                Print version information
```

### Environment Variables

The CLI supports the following environment variables:

- `SOMFY_API_KEY` - Your Somfy API key
- `SOMFY_GATEWAY_PIN` - Your TaHoma gateway PIN/ID

## Sample Output

The CLI provides detailed, colorized output showing all API interactions:

```
🔌 Testing connection to Somfy API...
✅ Successfully connected to API (protocol version: 3.7.2)

🏠 Discovering gateways...
✅ Found 1 gateway:
  🌐 Gateway: 0000-1111-2222 (Status: ALIVE, Protocol: 3.7.2)

🔍 Getting complete setup information...
✅ Setup contains 1 gateways and 3 devices

📱 Testing device discovery...
✅ Found 3 devices via get_devices():
  📱 Living Room Blinds (io://0000-1111-2222/12345678)
  📱 Bedroom Shutters (io://0000-1111-2222/87654321)
  📱 Kitchen Window (io://0000-1111-2222/11111111)

🔍 Testing device details and states...
✅ Device details for Living Room Blinds:
  🏷️  Label: Living Room Blinds
  🎛️  Type: io:StackComponent
  ⚡ States: 3, Attributes: 5
  
  📊 Device states:
    • core:StatusState: available
    • core:Memorized1PositionState: 50
    • core:MovingState: false

🎧 Testing event listener functionality...
✅ Successfully registered event listener with ID: 12345678-1234-5678-9012-123456789012
✅ Fetched events: []
✅ Successfully cleaned up event listener

⚡ Checking current executions...
✅ Found 0 current executions
  ℹ️  No executions currently running

🔧 Testing device filtering by controllable type...
✅ Found 2 devices with controllable type 'io:StackComponent':
  📱 io://0000-1111-2222/12345678
  📱 io://0000-1111-2222/87654321
```

## What the CLI Demonstrates

The CLI showcases all implemented API functionality:

### System Information
- API version retrieval and protocol verification
- Gateway discovery and connectivity status

### Device Management
- Complete device discovery
- Individual device details and capabilities
- Device state monitoring and retrieval
- Filtering devices by controllable type

### Event System
- Event listener registration and management
- Event fetching with automatic cleanup
- Proper listener lifecycle management

### Execution System
- Current execution monitoring
- Execution status tracking
- (Action execution available but disabled by default for safety)

### Error Handling
- Comprehensive error reporting
- User-friendly error messages
- Graceful handling of network issues and API errors

## Safety Features

The CLI includes several safety features to prevent accidental device actions:

- **Action execution is commented out by default** to prevent unintended device control
- **Clear confirmation prompts** for potentially destructive operations
- **Comprehensive logging** of all API interactions
- **Graceful error handling** with detailed error reporting

## Configuration

### Credentials

You need two pieces of information to use the CLI:

1. **API Key**: Your Somfy API authentication key
2. **Gateway PIN/ID**: Your TaHoma gateway identifier (format: `0000-1111-2222`)

### Connection Settings

The CLI automatically configures:
- **Protocol**: HTTPS for secure connections
- **Port**: 8443 (TaHoma Local API standard)
- **Certificate Handling**: Automatic handling of self-signed certificates
- **Timeout**: Reasonable timeouts for API calls

## Development

### Prerequisites

- Rust 1.70.0 or later
- Access to a Somfy TaHoma gateway
- Valid API credentials

### Building

```bash
# Build the CLI
cargo build -p cli

# Build with optimizations
cargo build --release -p cli
```

### Running in Development

```bash
# Run with cargo
cargo run -p cli

# Run with arguments
cargo run -p cli -- --api-key YOUR_KEY --gateway-pin YOUR_PIN
```

### Testing

```bash
# Run CLI tests
cargo test -p cli

# Run tests with output
cargo test -p cli -- --nocapture
```

## Architecture

The CLI is built on top of the SDK and demonstrates best practices for:

- **Async runtime management** with Tokio
- **Error handling** with comprehensive error reporting
- **Logging** with structured output
- **Configuration management** via environment variables and CLI args
- **User experience** with colorized, informative output

### Code Structure

```
cli/
├── src/
│   ├── main.rs                 # Main CLI entry point
│   ├── commands.rs             # Command definitions
│   ├── demo.rs                 # Demo functionality
│   └── lib.rs                  # Library root
├── Cargo.toml                  # Dependencies and metadata
└── README.md                   # This file
```

## Extending the CLI

To add new functionality:

1. **Add new commands** in `src/commands.rs`
2. **Extend demo scenarios** in `src/demo.rs`
3. **Update argument parsing** in `src/main.rs`
4. **Add tests** for new functionality

## Troubleshooting

### Common Issues

1. **Certificate errors**: The CLI handles self-signed certificates automatically
2. **Network timeouts**: Check your gateway connectivity and network settings
3. **Authentication errors**: Verify your API key and gateway PIN are correct
4. **Permission errors**: Ensure your API key has the necessary permissions

### Debug Mode

Run with detailed logging:

```bash
RUST_LOG=debug cargo run -p cli
```

## License

This project is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.