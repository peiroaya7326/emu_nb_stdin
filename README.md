# EmuNbStdin

EmuNbStdin is a simple library for simulating non-blocking standard input (stdin) in Rust without relying on OS-level non-blocking mechanisms. It utilizes internal synchronization mechanisms, condition variables, and channels to allow the main program to receive input from a background thread, suitable for scenarios where non-blocking behavior is required.

## Features

- Reads from standard input in a background thread, avoiding blocking the main logic.

- Uses synchronization tools such as Condvar and Mutex to ensure the correctness of data transmission.

- Provides a non-blocking poll method to check if data is available and a receive method to attempt to receive the data.

## Example Usage
```
use emu_nb_stdin::EmuNbStdin;

fn main() {
    let mut stdin = EmuNbStdin::new();

    loop {
        if stdin.poll() {
            if let Some(byte) = stdin.receive() {
                println!("Received byte: {}", byte);
            }
        }
        // Simulate other non-blocking work
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
```
## Known Issues

### Non-Guaranteed Data Availability

Please note that the poll method only checks if data is currently available, but the data may be consumed by the time receive is called (e.g., due to thread switching). This means that poll does not guarantee that receive will always successfully return data.