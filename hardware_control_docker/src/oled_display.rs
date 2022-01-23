//! This file contains the logic for setting up and controlling the OLED display.
//! 

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line, Rectangle},
    style::{PrimitiveStyleBuilder, TextStyleBuilder},
    DrawTarget,
};
use linux_embedded_hal::Spidev;
use ssd1306::{mode::GraphicsMode, Builder, I2CDIBuilder};
use ssd1306::prelude::*;
use ssd1306::prelude::SPIInterface;

use std::io;
use std::io::prelude::*;
use spidev::{ SpidevOptions, SpiModeFlags};

use tokio::sync::mpsc;

/// This enum contains all the commands available for controlling the display.
pub enum DisplayCommand {

}

/// Function for generating the SPI interface for a Raspberry Pi.
fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
         .bits_per_word(8)
         .max_speed_hz(20_000)
         .mode(SpiModeFlags::SPI_MODE_0)
         .build();
    spi.configure(&options)?;
    Ok(spi)
}



/// This task handles writing to the display and gets updates from the main program through a message queue.
pub async fn display_task ( manager_tx: mpsc::Receiver<DisplayCommand> ) {
    let spi = create_spi();
    //let interface = display_interface_spi::SPIInterface::new(spi, dc);

    //let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
    //    .into_buffered_graphics_mode();

    loop {

    }


}



