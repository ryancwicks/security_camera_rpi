//! This file contains the code for reading button presses.
//! 
//! The button is a normally 
use futures::stream::StreamExt;
use gpio_cdev::{AsyncLineEventHandle, Chip, EventRequestFlags, LineRequestFlags};

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

/// List of commands availabe for controlling the LED.
pub enum ButtonCommand {
    ButtonPressed,
}


pub async fn button_task (led_rx: mpsc::Sender<ButtonCommand>) {
    let mut gpio_chip = Chip::new("/dev/gpiochip0").expect ("Cannot setup GPIO for the system control button, exiting.");
    let input_button = gpio_chip
        .get_line(6).expect ("Could not access pin for system control button, exiting. ");
    let mut events = AsyncLineEventHandle::new(
        input_button.events (
            LineRequestFlags::INPUT | LineRequestFlags::ACTIVE_LOW, 
            EventRequestFlags::RISING_EDGE, 
            "gpioevents").unwrap())
            .unwrap();

    loop {
        match events.next().await {
            Some(event) => {
                match event {
                    Ok(_) =>{
                        sleep(Duration::from_millis(10)).await;
                        let button_read = input_button.request(LineRequestFlags::INPUT, 0, "read-input").unwrap();
                        if button_read.get_value().unwrap_or( 0 ) == 1 { //if still pressed after 10 ms then properly debounced.
                            led_rx.send(ButtonCommand::ButtonPressed).await.unwrap_or_else(|_| log::error!("Failed to send button pressed command to manager task."));
                        }
                    },
                    Err(e) => log::error!("Unexpected failed event from gpio bus {}.", e),
                }
            }, 
            None => break,
        };        
    }
}