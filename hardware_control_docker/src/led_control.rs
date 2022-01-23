//! This file contains the code for controlling the LED outputs.

use tokio::sync::mpsc;

/// List of commands availabe for controlling the LED.
pub enum LEDCommand {

}

pub async fn led_task (led_rx: mpsc::Receiver<LEDCommand>) {
    loop {

    }
}