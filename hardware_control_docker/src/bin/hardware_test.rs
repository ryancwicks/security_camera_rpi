use clap::Parser;

#[path="../disk_monitor.rs"]
mod disk_monitor;
use disk_monitor::DiskMonitor;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(short, long)]
    drive: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.drive.as_deref() {
        Some(drive) => {
            let monitor = DiskMonitor::new(&drive.to_owned());
            match monitor.get_disk_usage() {
                Ok ((used, full)) => {
                    println!("Drive size: {}/{} bytes", used, full );
                },
                Err(e) => {
                    println!("Error {}", e);
                }
            };
            
        }
        None => {
            println! ("Need to provide a drive to check.")
        }
    }
}