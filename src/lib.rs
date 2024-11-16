use std::io::{self, Read};
use std::sync::{mpsc, Arc, Condvar, Mutex};
use std::thread;

/// A non-blocking IO handler for reading from standard input without relying on OS-level non-blocking mechanisms.
///
/// This struct allows for simulating non-blocking IO by using internal synchronization and channels
/// to communicate between a reading thread and the main application.
pub struct EmuNbStdin {
    flag: Arc<(Mutex<bool>, Condvar)>,
    receiver: mpsc::Receiver<u8>,
}

impl EmuNbStdin {
    /// Creates a new instance of `EmuNbStdin` and starts a background thread to read from stdin.
    pub fn new() -> Self {
        let flag = Arc::new((Mutex::new(false), Condvar::new()));
        let (sender, receiver) = mpsc::channel();

        let flag_clone = Arc::clone(&flag);

        // Spawn a new thread to handle reading from stdin.
        let _read_stdio = thread::spawn(move || {
            let mut buffer = [0u8; 1];
            loop {
                if let Ok(bytes_read) = io::stdin().read(&mut buffer) {
                    if bytes_read == 0 {
                        break; // End of input.
                    }

                    let (ready, cvar) = &*flag_clone;
                    let mut ready = ready.lock().expect("Failed to lock mutex");
                    sender.send(buffer[0]).expect("Failed to send byte over channel");
                    *ready = true;
                    while *ready {
                        ready = cvar
                            .wait(ready)
                            .expect("Failed to wait on condition variable");
                    }
                }
                thread::sleep(std::time::Duration::from_millis(10)); // Avoid busy waiting.
            }
        });
        
        Self { flag, receiver }
    }

    /// Checks if there is a byte available to be read without blocking.
    ///
    /// Returns `true` if a byte is available, `false` otherwise.
    pub fn poll(&self) -> bool {
        let (ready, _cvar) = &*self.flag;
        let ready = ready.lock().expect("Failed to lock mutex");
        *ready
    }

    /// Attempts to receive a byte from the internal buffer without blocking.
    ///
    /// If a byte is available, returns `Some(u8)`, otherwise returns `None`.
    pub fn receive(&mut self) -> Option<u8> {
        let (ready, cvar) = &*self.flag;
        let mut ready = ready.lock().expect("Failed to lock mutex");

        match *ready {
            false => None,
            _ => {
                if let Ok(byte) = self.receiver.try_recv() {
                    *ready = false;
                    cvar.notify_one();
                    Some(byte)
                } else {
                    None
                }
            }
        }
    }
}


