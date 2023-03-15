use std::io::Write;
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use serialport::{DataBits, StopBits};
use tauri::Manager;

const PORT_NAME: &str = "/dev/tty.Bluetooth-Incoming-Port";
const BAUD_RATE: u32 = 9600;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_serial, start_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn read_serial<R: tauri::Runtime>(message: String, app_handle: &tauri::AppHandle<R>) {
  app_handle.emit_all("read_serial", message).unwrap();
}

#[tauri::command]
fn start_action(app_handle: tauri::AppHandle) {
  let (serial_output_tx, serial_output_rx) = mpsc::channel::<String>();

  let mut serial_port = serialport::new(PORT_NAME, BAUD_RATE)
      .timeout(Duration::from_millis(1000))
      .open()
      .expect("Failed to open serial port");

  let serial_output_tx_clone = serial_output_tx.clone();

  let finished_flag = Arc::new(Mutex::new(false));
  let finished_flag_clone = finished_flag.clone();

  thread::spawn(move || {
      let mut buffer = vec![0u8; 128];
      loop {
          let finished = finished_flag_clone.lock().unwrap();
          if *finished {
              break;
          }

          match serial_port.read(&mut buffer) {
              Ok(bytes_read) => {
                  let received_data = String::from_utf8_lossy(&buffer[..bytes_read]);
                  if received_data.trim() == "Finished" {
                      *finished_flag_clone.lock().unwrap() = true;
                  }
                  serial_output_tx_clone
                      .send(received_data.to_string())
                      .expect("Failed to send received data to channel");
              }
              Err(e) => {
                  eprintln!("Error reading from serial port: {:?}", e);
                  serial_output_tx_clone
                      .send(e.to_string())
                      .expect("Failed to send received data to channel");
              }
          }
      }
  });

  tauri::async_runtime::spawn(async move {
      while let Ok(output) = serial_output_rx.recv() {
          read_serial(output, &app_handle);
      }
  });
}

#[tauri::command]
fn write_serial(message: String) -> Result<String, String> {
    let build = serialport::new(PORT_NAME, BAUD_RATE)
        .data_bits(DataBits::Eight)
        .stop_bits(StopBits::One);

    let mut port = build.open().map_err(|e| e.to_string())?;
    let mut data = String::new();

    match port.write(message.as_bytes()) {
        Ok(_) => {
            data.push_str(&message);
            std::io::stdout().flush().unwrap();
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
        Err(e) => return Err(e.to_string())
    }

    Ok(data)
}