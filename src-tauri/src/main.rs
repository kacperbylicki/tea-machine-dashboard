use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::sync::mpsc;
use std::thread;
use serialport::{SerialPort, DataBits, StopBits};
use tauri::Manager;
use tauri::State;

// ResourceManager manages all the ports for the application.
struct ResourceManager {
    // Using Arc<Mutex<>> so that the ResourceManager can be safely shared and modified across threads.
    ports: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>,
}

// SerialPortSettings encapsulates all the settings required to open a serial port.
struct SerialPortSettings {
    port_name: String,
    baud_rate: u32,
    timeout: std::time::Duration,
}

impl ResourceManager {
    // Creates a new ResourceManager.
    fn new() -> Self {
        Self {
            ports: Mutex::new(HashMap::new()),
        }
    }

    // Provides a SerialPortSettings object for the first found "RaspberryPi Pico" product.
    // Otherwise, it defaults to an empty port_name, 19200 baud rate, and 5 seconds timeout.
    fn get_port_config() -> SerialPortSettings {
        // Defaults
        let mut settings: SerialPortSettings = SerialPortSettings {
            port_name: String::new(),
            baud_rate: 19200,
            timeout: std::time::Duration::from_millis(5000),
        };

        if let Ok(ports) = serialport::available_ports() {
            for port in ports {
                if let serialport::SerialPortType::UsbPort(usb_info) = port.port_type {
                    if let Some(product) = usb_info.product {
                        if product.contains("RaspberryPi Pico") {
                            // RaspberryPi Pico product found
                            settings.port_name = port.port_name;
                            settings.baud_rate = 19200;
                            settings.timeout = std::time::Duration::from_millis(1500);
                            break;
                        }
                    }
                }
            }
        }

        settings
    }

    // Adds a serial port to the resource manager.
    fn add_port(&self, id: String, port: Box<dyn SerialPort>) {
        let mut ports = self.ports.lock().unwrap();
        ports.insert(id, Arc::new(Mutex::new(port)));
    }

    // Retrieves a serial port from the resource manager.
    fn get_port(&self, id: &str) -> Option<Arc<Mutex<Box<dyn SerialPort>>>> {
        let ports = self.ports.lock().unwrap();
        ports.get(id).cloned()
    }
}

// The main function of the program.
fn main() {
    let resource_manager = Arc::new(ResourceManager::new());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_serial])
        .manage(resource_manager)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// Tauri command for writing to a serial port.
#[tauri::command]
fn write_serial(app_handle: tauri::AppHandle, message: String, port_id: String, state: State<'_, Arc<ResourceManager>>) -> Result<String, String> {
    // Debug message
    println!("Writing to serial port: {:?}", port_id);

    let resource_manager = state.inner();

    // Get port or create a new one if it doesn't exist
    let port = resource_manager.get_port(&port_id).unwrap_or_else(|| {
        let port_config = ResourceManager::get_port_config();
        let build = serialport::new(port_config.port_name, port_config.baud_rate)
            .data_bits(DataBits::Eight)
            .stop_bits(StopBits::One);

        let port = build.open().map_err(|e| e.to_string()).unwrap();
        resource_manager.add_port(port_id.clone(), port);
        resource_manager.get_port(&port_id).unwrap()
    });

    let mut port = port.lock().unwrap();

    // Write message to port
    match port.write(message.as_bytes()) {
        Ok(_) => {
            std::io::stdout().flush().unwrap();
            // Start reading from the port in a new thread
            start_action_impl(&app_handle, &resource_manager, &port_id, &message);
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
            // Handle timeout
            return Ok("TimedOut".to_string());
        },
        Err(e) => {
            // Handle other errors
            return Err(e.to_string());
        }
    }

    Ok("OK".to_string())
}

// start_action_impl starts reading from the serial port in a new thread.
fn start_action_impl(app_handle: &tauri::AppHandle, resource_manager: &Arc<ResourceManager>, port_id: &String, _base_message: &String) {
    let port = match resource_manager.get_port(port_id) {
        Some(port) => port,
        None => return,  // No port with the given ID
    };

    // Create a new channel
    let (tx, rx) = mpsc::channel();

    // Create a new thread for reading from the port
    let port_clone = Arc::clone(&port);
    thread::spawn(move || {
        let mut buffer = vec![0; 128];
        loop {
            let mut port = port_clone.lock().unwrap();
            match port.read(&mut buffer) {
                Ok(n) => {
                    // Send received data through the channel
                    let received_data = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if received_data.contains("OK") || received_data.contains("ERROR") {
                        if tx.send(received_data).is_err() {
                            eprintln!("Receiver has been dropped.");
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    continue;
                }
                Err(e) => {
                    eprintln!("Failed to read from port: {:?}", e);
                    break;
                }
            }
        }
    });

    // Process the received data in the async runtime
    let app_handle = app_handle.clone();

    tauri::async_runtime::spawn(async move {
        while let Ok(message) = rx.recv() {
            if message.trim().ends_with("-OK") {
                // Send the message to all windows
                app_handle.emit_all("read_serial", message).unwrap();
                continue;
            }
        }
    });
}
