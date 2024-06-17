use std::io::{Read};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::DownloaderError::Timeout;

// Enum to represent possible errors during the download process
enum DownloaderError {
    Timeout,
    Other(String),
}

// Downloader struct to hold the URL and timeout duration
struct Downloader {
    url: String,
    timeout: u64,
}

impl Downloader {
    // Constructor for Downloader struct
    fn new(url: &str, timeout: u64) -> Self {
        Downloader {
            url: url.to_string(),
            timeout,
        }
    }

    // Method to start the download process
    pub fn start(&self) -> Result<String, DownloaderError> {
        // Create a child process to run the curl command
        let child = Command::new("curl")
            .arg(&self.url)
            .stdout(Stdio::piped())
            .spawn()
            .map_err(|e| DownloaderError::Other(e.to_string()))?;

        // Wrap the child process in an Arc<Mutex<Child>> to share it between threads
        let mutex_child = Arc::new(Mutex::new(child));
        let mutex_child_clone = Arc::clone(&mutex_child);

        // Shared variable for the result
        let result = Arc::new(Mutex::new(None));
        let result_clone = Arc::clone(&result);

        // Timeout duration in seconds
        let timeout = self.timeout;

        // Spawn a timeout thread
        thread::spawn(move || {
            // Sleep for the duration of the timeout
            thread::sleep(Duration::from_secs(timeout));
            // Kill the child process if the timeout is reached
            let _ = mutex_child_clone.lock().unwrap().kill();
            // Set the result to a Timeout error
            *result_clone.lock().unwrap() = Some(Err(DownloaderError::Timeout));
        });

        // Allow the curl command some time to start
        thread::sleep(Duration::from_secs(2));
        let mut data = String::new();

        // Acquire the lock on the child process to read the stdout
        let mut child = mutex_child.lock().unwrap();
        let stdout = child.stdout.as_mut().unwrap();
        let mut buffer = [0; 1024];  // Buffer of 1024 bytes

        // Loop to read the output from the child process
        loop {
            // Check if the timeout thread has set a result
            if let Some(res) = result.lock().unwrap().take() {
                return res;
            }

            // Read data from the child process's stdout
            match stdout.read(&mut buffer) {
                Ok(0) => {
                    // End of output
                    break;
                }
                Ok(n) => {
                    // Append the read data to the output string
                    data.push_str(&String::from_utf8_lossy(&buffer[..n]));
                }
                Err(_) => {
                    // Handle read error
                    return Err(DownloaderError::Other("Failed to receive data".to_string()));
                }
            }
        }

        // Return the downloaded data
        Ok(data)
    }
}

fn main() {
    // Create a new Downloader instance with the URL and timeout duration
    let downloader = Downloader::new("http://www.google.com", 10);

    // Start the download and handle the result
    match downloader.start() {
        Ok(data) => {
            println!("Downloaded data: {} bytes", data.len());
        }
        Err(e) => match e {
            DownloaderError::Timeout => println!("Error: Timeout occurred"),
            DownloaderError::Other(err) => println!("Error: {}", err),
        },
    }
}
