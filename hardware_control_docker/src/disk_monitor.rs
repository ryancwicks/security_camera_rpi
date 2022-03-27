/// Structure for monitoring the systems disk usage.
use sysinfo::{System, SystemExt, Disk, DiskExt};
use std::error::Error;

pub fn get_disk_usage(disk_path: &String) -> Result<(u64, u64), Box<dyn Error + Send + Sync>> {
    let disk_path = disk_path.to_string();
    let mut sys = System::new_all();
    
    // First we update all information of our `System` struct.
    sys.refresh_all();
    let mut disk: Option<&mut Disk> = None;
    for a_disk in sys.disks_mut() {
        if a_disk.name() == disk_path.as_str() {
            disk = Some(a_disk);
            break;
        }
    }
    match disk {
        Some(val) => {
            if val.refresh() {
                Ok((val.available_space(), val.total_space()))
            } else {
                Err("Failed to update drive info.".into())
            }
        },
        None => Err("Failed to find drive.".into())
    }
}   
