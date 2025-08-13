use clap::Parser;
use log::debug;
use somfy_sdk::api_client::ApiClient;

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
    let res_device = api_client.get_device("io://0812-2424-9999/12936651").await;
    debug!("{res_version:?}");
    debug!("{res_gateways:?}");
    debug!("{res_devices:?}");
    debug!("{res_device:?}");

    Ok(())
}

async fn _run_demo(api_client: ApiClient) {
    println!("🔗 Connecting to Somfy TaHoma gateway...");

    // Get API version
    match api_client.get_version().await {
        Ok(version) => println!("✅ API Version: {}", version.protocol_version),
        Err(e) => println!("❌ Failed to get version: {e:?}"),
    }

    // Get complete setup information
    println!("\n📋 Getting complete setup information...");
    match api_client.get_setup().await {
        Ok(setup) => {
            println!(
                "✅ Setup contains {} gateways and {} devices",
                setup.gateways.len(),
                setup.devices.len()
            );

            // Display gateway information
            for gateway in &setup.gateways {
                println!(
                    "  🏠 Gateway: {} (Status: {})",
                    gateway.gateway_id, gateway.connectivity.status
                );
            }

            // Display device information and demonstrate additional calls
            for device in &setup.devices {
                println!(
                    "  📱 Device: {} - {} ({})",
                    device.label, device.controllable_name, device.device_url
                );

                // Get device states for each device
                match api_client.get_device_states(&device.device_url).await {
                    Ok(states) => {
                        println!("    🔍 Device has {} states:", states.len());
                        for state in &states {
                            println!("      • {}: {:?}", state.name, state.value);
                        }

                        // Get a specific device state (if any exist)
                        if let Some(first_state) = states.first() {
                            match api_client
                                .get_device_state(&device.device_url, &first_state.name)
                                .await
                            {
                                Ok(specific_state) => {
                                    println!(
                                        "    🎯 Specific state '{}': {:?}",
                                        specific_state.name, specific_state.value
                                    );
                                }
                                Err(e) => println!("    ❌ Failed to get specific state: {e:?}"),
                            }
                        }
                    }
                    Err(e) => println!("    ❌ Failed to get device states: {e:?}"),
                }
            }

            // Demonstrate getting devices by controllable type
            if let Some(first_device) = setup.devices.first() {
                println!(
                    "\n🔍 Finding devices with controllable type '{}'...",
                    first_device.controllable_name
                );
                match api_client
                    .get_devices_by_controllable(&first_device.controllable_name)
                    .await
                {
                    Ok(device_urls) => {
                        println!(
                            "✅ Found {} devices with this controllable type:",
                            device_urls.len()
                        );
                        for device_url in device_urls {
                            println!("  • {device_url}");
                        }
                    }
                    Err(e) => println!("❌ Failed to get devices by controllable: {e:?}"),
                }
            }
        }
        Err(e) => println!("❌ Failed to get setup: {e:?}"),
    }

    // Individual API demonstrations
    println!("\n🔧 Individual API demonstrations...");

    // Get gateways specifically
    match api_client.get_gateways().await {
        Ok(gateways) => {
            println!("✅ Found {} gateways via get_gateways():", gateways.len());
            for gateway in gateways {
                println!(
                    "  🏠 {} - Protocol: {}",
                    gateway.gateway_id, gateway.connectivity.protocol_version
                );
            }
        }
        Err(e) => println!("❌ Failed to get gateways: {e:?}"),
    }

    // Get devices specifically
    match api_client.get_devices().await {
        Ok(devices) => {
            println!("✅ Found {} devices via get_devices():", devices.len());
            for device in devices {
                println!("  📱 {} ({})", device.label, device.device_url);
            }
        }
        Err(e) => println!("❌ Failed to get devices: {e:?}"),
    }

    // Get a specific device (using a known device URL if available)
    let sample_device_url = "io://0812-2424-9999/12936651";
    match api_client.get_device(sample_device_url).await {
        Ok(device) => {
            println!(
                "✅ Got specific device '{}' with {} states and {} attributes",
                device.label,
                device.states.len(),
                device.attributes.len()
            );
        }
        Err(e) => println!("ℹ️  Could not get sample device '{sample_device_url}': {e:?}"),
    }

    // Event listener demonstration
    println!("\n🎧 Event listener demonstration...");
    match api_client.register_event_listener().await {
        Ok(listener) => {
            println!("✅ Successfully registered event listener with ID: {}", listener.id);
            println!("   ℹ️  Listener will automatically expire after 10 minutes of inactivity");
            
            // Demonstrate fetching events
            match api_client.fetch_events(&listener.id).await {
                Ok(_) => {
                    println!("   ✅ Successfully fetched events for listener {}", listener.id);
                }
                Err(e) => println!("   ⚠️  Could not fetch events: {e:?}"),
            }
            
            // Demonstrate unregistering the listener
            match api_client.unregister_event_listener(&listener.id).await {
                Ok(_) => {
                    println!("   ✅ Successfully unregistered event listener {}", listener.id);
                }
                Err(e) => println!("   ❌ Failed to unregister event listener: {e:?}"),
            }
        }
        Err(e) => println!("❌ Failed to register event listener: {e:?}"),
    }

    // Execution demonstration
    println!("\n⚡ Execution demonstration...");
    
    // Get current executions
    match api_client.get_current_executions().await {
        Ok(executions) => {
            println!("✅ Found {} current executions", executions.len());
            for execution in &executions {
                println!("  🔄 Execution: {}", execution.id);
                
                // Demonstrate getting specific execution details
                match api_client.get_execution(&execution.id).await {
                    Ok(exec_details) => {
                        println!("    ✅ Retrieved details for execution {}", exec_details.id);
                    }
                    Err(e) => println!("    ❌ Failed to get execution details: {e:?}"),
                }
            }
            
            if !executions.is_empty() {
                println!("   ℹ️  You can use cancel_execution() to cancel specific executions");
                println!("   ℹ️  You can use cancel_all_executions() to cancel all executions");
            }
        }
        Err(e) => println!("❌ Failed to get current executions: {e:?}"),
    }

    // Example of how to use execute_actions (commented out to avoid unwanted actions)
    println!("\n💡 Action execution example (not actually executed):");
    println!("   // Example: Turn on a light");
    println!("   // let actions = vec![Action {{");
    println!("   //     device_url: \"io://0812-2424-9999/12936651\".to_string(),");
    println!("   //     commands: vec![Command {{");
    println!("   //         name: \"on\".to_string(),");
    println!("   //         parameters: vec![],");
    println!("   //     }}],");
    println!("   // }}];");
    println!("   // let request = ExecuteRequest {{");
    println!("   //     label: Some(\"Turn on light\".to_string()),");
    println!("   //     actions,");
    println!("   // }};");
    println!("   // match api_client.execute_actions(request).await {{");
    println!("   //     Ok(execution_id) => println!(\"✅ Started execution: {{}}\", execution_id.id),");
    println!("   //     Err(e) => println!(\"❌ Failed to execute actions: {{e:?}}\"),");
    println!("   // }}");

    println!("\n🎉 CLI demonstration complete!");
}
