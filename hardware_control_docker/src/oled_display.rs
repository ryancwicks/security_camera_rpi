//! This file contains the logic for setting up and controlling the OLED display.
//! 

use linux_embedded_hal::{Spidev, CdevPin};
use ssd1306::{prelude::*, Ssd1306};
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text},
    
};
use std::io;
use spidev::{ SpidevOptions, SpiModeFlags};
use gpio_cdev::{Chip, LineRequestFlags};
use tokio::sync::mpsc;

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
/// 
/// TODO fix all the unwraps
pub async fn display_task ( mut manager_tx: mpsc::Receiver<String>) {
    
    let spi = create_spi().unwrap();

    //Need to setup d/c (gpio08) pin, reset pin (gpio24) and CS pin (gpio23).
    // /dev/gpiochip0 maps to the driver for the SoC (builtin) GPIO controller.
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let dc_cdev = chip
        .get_line(8).unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "display-dc").unwrap();
    let dc = CdevPin::new(dc_cdev).unwrap();
    let cs_cdev = chip
        .get_line(23).unwrap()
        .request(LineRequestFlags::OUTPUT, 1, "display-cs").unwrap();
    let cs = CdevPin::new(cs_cdev).unwrap();
    let rst_cdev = chip
        .get_line(24).unwrap()
        .request(LineRequestFlags::OUTPUT, 0, "display-rst").unwrap();
    let _rst = CdevPin::new(rst_cdev).unwrap();

    let interface = display_interface_spi::SPIInterface::new(spi, dc, cs);
    let mut display = Ssd1306::new(interface, DisplaySize128x32, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();

    loop {
        match manager_tx.recv().await {
            Some(val) => {
                //Has to be reinitialized each time because MonoTextStyle isn't send, can't exist across an await.
                let text_style = MonoTextStyleBuilder::new()
                    .font(&FONT_6X10)
                    .text_color(BinaryColor::On)
                    .build();
                
                Text::with_baseline(&val, Point::zero(), text_style, Baseline::Top)
                    .draw(&mut display)
                    .unwrap();
            },
            None =>  () 
        };
    }
}



