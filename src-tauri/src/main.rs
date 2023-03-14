#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::io::Write;
use std::time::Duration;
use serialport::{DataBits, StopBits};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![
      read_serial,
      write_serial
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn read_serial() -> Result<String, String> {
    let portname = "/dev/tty.Bluetooth-Incoming-Port";
    let baudrate = 115200;

    let port = serialport::new(portname,baudrate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port)=>{
            let mut serialbuf: Vec<u8> = vec![0;100];
            let mut data = String::new();
            loop{
                match port.read(serialbuf.as_mut_slice()){
                    Ok(t) => {
                        data.push_str(std::str::from_utf8(&serialbuf[..t]).unwrap());
                    },
                    Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => return Err(e.to_string())
                }
            }
            Ok(data)
        }

        Err(e)=>{
            Err(e.to_string())
        }
    }
}

#[tauri::command]
fn write_serial() -> Result<String, String> {
    let portname = "/dev/tty.Bluetooth-Incoming-Port";
    let baudrate = 115200;
    let databits = DataBits::Eight;
    let stopbits = StopBits::One;
    let string   = "rust";

    let build = serialport::new(portname,baudrate)
        .data_bits(databits)
        .stop_bits(stopbits);

    let mut port = build.open().map_err(|e| e.to_string())?;

    let mut data = String::new();

    match port.write(string.as_bytes()) {
      Ok(_) => {
          data.push_str(string);
          std::io::stdout().flush().unwrap();
      },
      Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
      Err(e) => return Err(e.to_string())
  }

  return Ok(data)
}