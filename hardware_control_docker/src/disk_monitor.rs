/// Structure for monitoring the systems disk usage.
use sysinfo::{System, SystemExt, Disk, DiskExt};
use std::error::Error;

pub struct DiskMonitor {
    disk_path: String,
}

impl DiskMonitor {
    pub fn new(disk_path: &String) -> Self {
        DiskMonitor {
            disk_path: disk_path.to_string()
        }
    }

    pub fn get_disk_usage(self: &Self) -> Result<(u64, u64), Box<dyn Error>> {
    
        let mut sys = System::new_all();
        // First we update all information of our `System` struct.
        sys.refresh_all();

        let mut disk: Option<&mut Disk> = None;
        for a_disk in sys.disks_mut() {
            if a_disk.name() == self.disk_path.as_str() {
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

}
