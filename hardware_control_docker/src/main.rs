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
//!  | Pin 19     | SPI_MOSI | To SSD1306 MISO |
//!  | Pin 21     | SPI_MISO | To SSD1306 MOSI |
//!  | Pin 23     | SPI_CLK  | To SSD1306 SCK  |
//!  | Pin 24     | SPI_CE_0_N | Chip Select for SSD1306 |
//!  | Pin 17     | 3.3V     | Powers the LED, switch and Display (Vin)|
//!  | Pin 25     | Ground   | Ground for all signals |
//! 
//! This software is run inside a container, and that container needs to be run with the --priviledged tag.
//! 

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://hyper.rs").await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}