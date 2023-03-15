# Serial Communication with Tauri Application

This code demonstrates serial communication with a Bluetooth device using Rust and the Tauri framework for creating a cross-platform desktop application. It defines three main functions: `main`, `start_action`, and `write_serial`.

## Dependencies

To use this code, you will need the following dependencies in your `Cargo.toml` file:

```
[dependencies]
tauri = "1.0"
serialport = "4.0"
```

## Constants

```
const PORT_NAME: &str = "/dev/tty.Bluetooth-Incoming-Port";
const BAUD_RATE: u32 = 9600;
```

PORT_NAME: The name of the serial port that the Bluetooth device is connected to.
BAUD_RATE: The baud rate for the serial communication.

## Functions

### main()

```
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![write_serial, start_action])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

The main function initializes the Tauri application, registering the write_serial and start_action functions as Tauri command handlers.

### read_serial()

```

fn read_serial<R: tauri::Runtime>(message: String, app_handle: &tauri::AppHandle<R>) {
    app_handle.emit_all("read_serial", message).unwrap();
}

```

The read_serial function emits a "read_serial" event to the Tauri application with the received message from the serial port.

### start_action()

```

#[tauri::command]
fn start_action(app_handle: tauri::AppHandle) { ... }

```

The start_action function is a Tauri command that initializes the serial communication by opening the serial port and starting a separate thread for reading data from the port. It also listens for incoming data on a channel and emits a "read_serial" event with the received data.

### write_serial()

```
#[tauri::command]
fn write_serial(message: String) -> Result<String, String> { ... }
```

The write_serial function is a Tauri command that writes a given message to the serial port. It returns the written message if successful, or an error message if there was a problem writing to the port.

## Usage

To use this code in your Tauri application:

- Include the code in your main.rs file.
- Register the write_serial and start_action functions as Tauri command handlers in the main function.
- In your Tauri application frontend, use the Tauri API to invoke the write_serial and start_action functions.
- Listen for the "read_serial" event in your frontend to receive data from the serial port.
