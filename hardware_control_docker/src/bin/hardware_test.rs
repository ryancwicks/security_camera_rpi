use clap::Parser;
use tokio::sync::mpsc;

#[path="../disk_monitor.rs"]
mod disk_monitor;
use disk_monitor::get_disk_usage;

#[path="../oled_display.rs"]
mod oled_display;
use oled_display::display_task;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    drive: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let hard_drive = match cli.drive.as_deref() {
        Some(drive) => drive,
        None => "/dev/sdb2"
    };

    let (tx_screen, rx_screen) = mpsc::channel(1);
    tokio::spawn(async move {
        display_task(rx_screen).await;
    });

    print!("Hello World!");
    let _ret = tx_screen.send("Hello World!".to_string());

    let _ret = tokio::time::sleep(std::time::Duration::from_secs(5));

    match get_disk_usage(&hard_drive.to_owned()) {
        Ok ((available, full)) => {
            println!("Drive size: {}% full.", (full-available) as f64/ full as f64 );
            let _ret = tx_screen.send(format!("Drive size: {}% full.", (full-available) as f64/ full as f64 ));
        },
        Err(e) => {
            println!("Error {}", e);
            let _ret = tx_screen.send(format!("Error {}", e));
        }
    };


}