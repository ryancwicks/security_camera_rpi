//!
//! Hardware Control
//! 
//! This program is used by the security system server running on a Raspberry Pi to control the SSD1306 display, read the state of input buttons,
//! blink LED's when the security system is recording and starting and stopping recording based on button presses (communicating with moonfire-nvr)
//! 
//! The hardware setup is uses the following GPIO's
//! 
//!  | Header Pin | Name     | Pin Function | 
//!  |------------|----------|--------------|
//!  | Pin 29     | GPIO05   | Indicator LED driving the base of an NPN transistor (Inverted Logic) |
//!  | Pin 31     | GPIO06   | Start/Top Recording button (pulled high normally, pulled low when pressed, not debounced) |
//!  | Pin 21     | SPI_MOSI (GPIO10) | To SSD1306 DATA |
//!  | Pin 23     | SPI_CLK (GPIO11) | To SSD1306 SCK  |
//!  | Pin 24     | SPI_CE_0_N (GPIO08) | Chip Select for SSD1306 |
//!  | Pin 17     | 3.3V     | Powers the LED, switch and Display (Vin)|
//!  | Pin 25     | Ground   | Ground for all signals |
//!  | Pin 16     | GPIO23   | D/C |
//!  | Pin 18     | GPIO24   | RST |
//! 
//! This software is run inside a container, and that container needs to be run with the --priviledged tag.
//! 

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio::signal;

mod oled_display;
mod led_control;
mod button_handler;
use oled_display::{display_task, DisplayCommand};
use led_control::{led_task, LEDCommand};
use button_handler::{button_task, ButtonCommand};

#[derive(Debug)]
enum Commands {
    StartRecording,
    StopRecording,
    Shutdown,
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    log::info!("Security System Hardware Control Started.");
    let (tx_led, rx_led) = mpsc::channel(1);
    let (tx_screen, rx_screen) = mpsc::channel(1);
    let (tx_button, rx_button) = mpsc::channel(1);

    tokio::spawn(async move {
        led_task(rx_led).await;
    });
    tokio::spawn(async move {
        display_task(rx_screen).await;
    });
    tokio::spawn(async move {
        button_task(tx_button).await;
    });

    tokio::spawn( async move {
        manager(tx_led, tx_screen, rx_button).await;
    });

    shutdown_signal().await;
}

async fn manager(led_tx: mpsc::Sender<LEDCommand>, display_tx: mpsc::Sender<DisplayCommand>, button_rx: mpsc::Receiver<ButtonCommand>) {
    loop {

    }
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");
}