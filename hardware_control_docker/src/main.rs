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
//!  | Pin 25     | GPIO25   | Chip Select for SSD1306 |
//!  | Pin 17     | 3.3V     | Powers the LED, switch and Display (Vin)|
//!  | Pin 25     | Ground   | Ground for all signals |
//!  | Pin 16     | GPIO23   | D/C |
//!  | Pin 18     | GPIO24   | RST |
//! 
//! This software is run inside a container, and that container needs to be run with the --priviledged tag.
//! 
use clap::Parser;
use tokio::sync::mpsc;

mod oled_display;
mod led_control;
mod button_handler;
pub mod disk_monitor;

use oled_display::display_task;
//use led_control::led_task;
use button_handler::{ButtonCommand};
use disk_monitor::get_disk_usage;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    drive: Option<String>,
}

//#[derive(Debug)]
//enum Commands {
//    StartRecording,
//    StopRecording,
//    Shutdown,
//}

#[derive(Debug)]
pub enum SystemState {
    Idle,
    Recording,
    SystemError(String),
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let cli = Cli::parse();

    let main_storage = match cli.drive {
        Some(val) => val,
        None => "/dev/sda2".to_string()
    };

    log::info!("Security System Hardware Control Started.");
    let (tx_led, _rx_led) = mpsc::channel(1);
    let (tx_screen, rx_screen) = mpsc::channel(1);
    let (_tx_button, rx_button) = mpsc::channel(1);

    //tokio::spawn(async move {
    //    led_task(rx_led).await;
    //});

    tokio::spawn(async move {
        display_task(rx_screen).await;
    });

    //tokio::spawn(async move {
    //    button_task(tx_button).await;
    //});

    tokio::spawn( async move {
        manager(tx_led, tx_screen, rx_button, &main_storage).await;
    });

    // Wait for the CTRL+C signal
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to install CTRL+C signal handler");
}

async fn manager(_led_tx: mpsc::Sender<SystemState>, display_tx: mpsc::Sender<String>, _button_rx: mpsc::Receiver<ButtonCommand>, main_storage: &String) {

    let mut _system_state = SystemState::Idle;

    loop {
        match get_disk_usage(main_storage) {
            Ok((available, total)) => {
                log::info!("In Main: \nCottage Recorder\nDisk Usage: {:.3}%", (total-available) as f64/ total as f64);
                display_tx.send(format!("Cottage Recorder\nDisk Usage: {:.3}%", (total-available) as f64/ total as f64 )).await.unwrap();
            },
            Err(e) => {
                log::error!("Error: {}", e);
                display_tx.send(format!("Error: {}", e)).await.unwrap();
            }
        };
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}


