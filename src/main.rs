use std::{error::Error, thread};
use std::time::{SystemTime, Duration};

use env_logger;
use log::{log, Level};

use timekeeper::window::get_current_window;

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let poll_interval = Duration::from_secs(1); //TODO: poll_interval -> parameter
    let mut now: SystemTime = SystemTime::now();
    let mut current_window: Option<String> = None;
    // TODO: Create a way to exit or change to server-client style
    // TODO: Create a way to store and manage elapsed times -> HashMap?
    loop {
        // Get window name
        let output = match get_current_window() {
            Ok(window)  => Some(window),
            Err(err)    => {
                log!(Level::Error, "get_current_window failed to grab name! Error: {:?}", err);
                None
            },
        };
        
        if !current_window.is_some() {
            current_window = output.clone();
            now = SystemTime::now();
        }

        if current_window != output {
            current_window = output.clone();
            now = SystemTime::now();
        }

        log!(Level::Info, "Focused on {}. Elapsed time: {}", current_window.clone().unwrap(), now.elapsed()?.as_secs());

        thread::sleep(poll_interval);
    }
}
