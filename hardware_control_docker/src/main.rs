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
//!  | Pin      | GPIO   | Indicator LED driving the base of an NPN transistor (Inverted Logic) |
//!  | Pin      | GPIO   | Start/Top Recording button (pulled high normally, pulled low when pressed, not debounced) |
//!  | Pin 21     | SPI_MISO (GPIO09) | To SSD1306 MOSI (DATA) |
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

#[derive(Debug)]
enum Commands {
    StartRecording,
    StopRecording,
    Shutdown,
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {

    let (tx_led, mut rx_from_tasks) = mpsc::channel(32);
    let tx_screen = tx_led.clone();
    let tx_button = tx_led.clone();

    let task_led = tokio::spawn(async move {
        led_handler(tx_led).await;
    });
    let task_screen = tokio::spawn(async move {
        display_handler(tx_screen).await;
    });
    let task_button = tokio::spawn(async move {
        button_handler(tx_button).await;
    });

    let task_manager = tokio::spawn( async move {
        manager(rx_from_tasks).await;
    });

    task_led.await.unwrap();
    task_screen.await.unwrap();
    task_button.await.unwrap();
    task_manager.await.unwrap();

}

async fn manager(manager_rx: mpsc::Receiver<Commands>) {

}

async fn button_handler(manager_tx: mpsc::Sender<Commands>) {

    sleep(Duration::from_millis(1000)).await;

}

async fn led_handler(manager_tx: mpsc::Sender<Commands>) {
    sleep(Duration::from_millis(1000)).await;
}

async fn display_handler(manager_tx: mpsc::Sender<Commands>) {
    sleep(Duration::from_millis(1000)).await;
}