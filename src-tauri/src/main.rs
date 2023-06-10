use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::io::Write;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;
use serialport::{SerialPort, DataBits, StopBits};
use tauri::Manager;
use tauri::State;
use serde::Deserialize;
use std::thread::sleep;

const PORT_NAME: &str = "/dev/tty.usbmodem2101";
const BAUD_RATE: u32 = 19200;

#[derive(Deserialize)]
struct RecipeStep {
    message: String,
    delay_in_seconds: u64,
}

#[derive(Deserialize)]
struct WriteSerialMessage {
    recipe: Vec<RecipeStep>,
    port_id: String,
}

struct ResourceManager {
    ports: Mutex<HashMap<String, Arc<Mutex<Box<dyn SerialPort>>>>>,
}

impl ResourceManager {
    fn new() -> Self {
        Self {
            ports: Mutex::new(HashMap::new()),
        }
    }

    fn add_port(&self, id: String, port: Box<dyn SerialPort>) {
        let mut ports = self.ports.lock().unwrap();
        ports.insert(id, Arc::new(Mutex::new(port)));
    }

    fn get_port(&self, id: &str) -> Option<Arc<Mutex<Box<dyn SerialPort>>>> {
        let ports = self.ports.lock().unwrap();
        ports.get(id).cloned()
    }

    fn close_port(&self, id: &str) {
        let mut ports = self.ports.lock().unwrap();
        ports.remove(id);
        drop(ports);
    }
}

fn main() {
    let resource_manager = Arc::new(ResourceManager::new());

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_serial])
        .manage(resource_manager)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn write_serial(app_handle: tauri::AppHandle, msg: WriteSerialMessage, state: State<'_, Arc<ResourceManager>>) -> Result<String, String> {
    println!("Writing to serial port: {:?}", msg.port_id);

    let resource_manager = state.inner();

    let port = match resource_manager.get_port(&msg.port_id) {
        Some(port) => port,
        None => {
            let build = serialport::new(PORT_NAME, BAUD_RATE)
                .data_bits(DataBits::Eight)
                .stop_bits(StopBits::One);

            let port = build.open().map_err(|e| e.to_string())?;
            
            resource_manager.add_port(msg.port_id.clone(), port);
            resource_manager.get_port(&msg.port_id).unwrap()
        }
    };

    for step in msg.recipe {
        let mut port = port.lock().unwrap();

        println!("Writing \"{}\" to serial port", step.message);

        match port.write(step.message.as_bytes()) {
            Ok(_) => {
                std::io::stdout().flush().unwrap();
                // Call start_action directly
                start_action_impl(&app_handle, &resource_manager, &msg.port_id, &step.message);

                // Delay execution as per the recipe step
                sleep(Duration::from_secs(step.delay_in_seconds));
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                return Ok("TimedOut".to_string());
            },
            Err(e) => {
                return Err(e.to_string());
            }
        }
    }

    resource_manager.close_port(&msg.port_id);
    
    Ok("OK".to_string())
}

fn start_action_impl(app_handle: &tauri::AppHandle, resource_manager: &Arc<ResourceManager>, port_id: &String, base_message: &String) {
    let port = match resource_manager.get_port(port_id) {
        Some(port) => port,
        None => return,  // No port with the given ID
    };

    let (tx, rx) = mpsc::channel();

    let port_clone = Arc::clone(&port);
    thread::spawn(move || {
        let mut buffer = vec![0; 128];
        loop {
            let mut port = port_clone.lock().unwrap();
            match port.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        continue;
                    }
                    let received_data = String::from_utf8_lossy(&buffer[..n]).to_string();
                    println!("Received data: {}", received_data);
                    tx.send(received_data).expect("Failed to send data to channel");
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => {
                    // If it's just a timeout, continue the loop without breaking.
                    continue;
                }
                Err(e) => {
                    eprintln!("Failed to read from port: {:?}", e);
                    break;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Clone the AppHandle and the base_message to use inside the async closure
    let app_handle = app_handle.clone();
    let base_message = base_message.clone();

    tauri::async_runtime::spawn(async move {
        while let Ok(message) = rx.recv() {
            println!("Received message: {}", message);
            if message.trim() == format!("{}-OK", &base_message) {
                app_handle.emit_all("read_serial", message).unwrap();
                break;
            }
        }
    });
}