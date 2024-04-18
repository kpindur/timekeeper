use std::{error::Error, process::Command};

use log::{log, Level};

///
///
pub fn get_current_window() -> Result<String, Box<dyn Error>> {
    // xprop -root _NET_ACTIVE_WINDOW
    let output = Command::new("xprop")
        .arg("-root")
        .arg("_NET_ACTIVE_WINDOW")
        .output()?;

    let window_id = String::from_utf8(output.stdout)?;
    let window_id = window_id.split_whitespace().last().expect("Could not extract window id!");
    log!(Level::Info, "Extracted process ID: {}", window_id);

    let output = Command::new("xprop")
        .arg("-id")
        .arg(window_id)
        .arg("_NET_WM_PID")
        .output()?;
    
    let process_id = String::from_utf8(output.stdout)?;
    let process_id = process_id.split_whitespace().last().expect("Could not extract process id!");
    log!(Level::Info, "Extracted process name: {}", process_id);

    let output = Command::new("ps")
        .arg("-p")
        .arg(process_id)
        .arg("-o")
        .arg("comm=")
        .output()?;

    let process_name = String::from_utf8(output.stdout)?;
    log!(Level::Info, "Extracted window name: {}", process_name);

    Ok(process_name)
}
