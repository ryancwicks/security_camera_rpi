# cottage-security-system

## Overview

Security Camera system built around a Raspberry Pi 4 and some IP security cameras. The main application is using [moonfire-nvr](https://github.com/scottlamb/moonfire-nvr/). As much as possible, I set up the system around a base Raspian image and used Docker containers for the system services (DHCP, WIFI hot spot, moonfire-nvr, etc).

## Setup

Be sure to attach the drive to the Raspberry Pi during setup.

### Raspian Setup

First, download the Raspian Lite image from [here](https://downloads.raspberrypi.org/raspios_lite_armhf/images/raspios_lite_armhf-2021-05-28/2021-05-07-raspios-buster-armhf-lite.zip). 

Flash this image onto your SD card (the command below worked for the image and particular SD card used, change your command appropriately):

```
sudo dd if=2021-05-07-raspios-buster-armhf-lite.img of=/dev/sdb bs=4M conv=fsync
```

On the SD card, I added a file called ssh to the boot partition This will allow me to plug into the network and log into the device without a screen.

Open cmdline.txt in the /boot partition and add net.ifnames=0 to the first line.

Log into the system through ssh.

Run raspi-config:

```
sudo raspi-config
```

In the raspi-config tool:
    - Change the default password (System->Password)
    - Change the system hostname (System->Hostname)(security-camera-server in this case)
    - Set the timezone and wifi country in Localization.
    - Set up wifi to your local network for downloading packages.

Install required packages

```
sudo apt update
sudo apt upgrade
sudo apt install vim ca-certificates curl gnupg lsb-release
curl -fsSL https://download.docker.com/linux/debian/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/debian \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
sudo apt update
sudo apt install docker-ce docker-ce-cli containerd.io
sudo usermod -aG docker $USER
sudo apt install libffi-dev libssl-dev python3 python3-pip
sudo pip3 install docker-compose
sudo apt install i2c-tools
```


### Hard Drive Setup

Determine the if the USB drive is using UAS:

```
lsusb -t
```

Look for driver UAS. If it is there, it needs to be disabled. Use lsusb to get the id.

Edit the /boot/cmdline.txt file and add the following to the start of the line of parameters:

```
usb-storage.quirks=0bc2:ac39:u
```

Then restart the system.

Create a ext4 partition on the large data hard drive (a 5TG USB3 drive in my case), first deleting the existing partition.

```
sudo fdisk /dev/sda #In my case
#delete existing partitions and create a new one of 10 GB and another filling the rest of the drive.
mkfs.ext4 -T largefile -m 1 /dev/sdc1
mkfs.ext4 -T largefile -m 1 /dev/sdc2
```

Now create a mount point and set it up. First get the UUID for the USB drive:

```
sudo blkid
```

Now add this line to fstab, and follow the next steps to create and test the mount point.

```
sudo mkdir /var/lib/moonfire-nvr
sudo mkdir /media/nvr
sudo vi /etc/fstab
```

Add the following line to the bottom of the fstab file:

```
UUID=<Your UUID for the small drive> /var/lib/moonfire-nvr ext4 nofail,noatime,lazytime,data=writeback,journal_async_commit  0  2
UUID=<Your UUID for the large drive> /media/nvr ext4 nofail,noatime,lazytime,data=writeback,journal_async_commit  0  2
```

You should now be able to mount the /media/nvr directory.

### Adding the moonfire-nvr User

Create the moonfire-nvr user on the USB drive. This will prevent the SD card from being work out, and also allow the moonfire sql database to be created. 

'''
sudo useradd --user-group --create-home --home /var/lib/moonfire-nvr moonfire-nvr
'''

Adjust the time zone and copy the conf_files/nvr to /usr/local/bin/nvr.

Generate the database for nvr by running:

```
sudo nvr init
```

### Setting up the data store

```
sudo install -d -o moonfire-nvr -g moonfire-nvr -m 700 /media/nvr/sample
sudo chown -R moonfire-nvr:moonfire-nvr /media/nvr
```

Now that the mount is setup, you need to modify the /usr/local/bin/nvr script to properly start the nvr (uncomment the mountpoints under "Additional Mount Lines"

### Network Setup

Copy the interfaces file from hostapd_docker/confs/interfaces to /etc/interfaces. Restart the networking.

### Setting up the RTC

I added a DS1307 RTC module (instruction from [here](https://wiki.52pi.com/index.php?title=DS1307_RTC_Module_with_BAT_for_Raspberry_Pi_SKU:_EP-0059)). 

please ensure that /boot/config.txt file include two paramaters:

```
dtoverlay=i2c-rtc,ds1307 
dtparam=i2c_arm=on
```

After that, please make sure you have disabled the "fake hwclock" which interferes with the 'real' hwclock

```
sudo apt-get -y remove fake-hwclock
sudo update-rc.d -f fake-hwclock remove
```

Now with the fake-hw clock off, you can start the original 'hardware clock' script.
Edit the script file /lib/udev/hwclock-set with nano or vim editor and comment out these three lines:

```
if [ -e /run/systemd/system ] ; then
 exit 0
fi
```

Restart and then verify the RTC is found using i2cdetect -y 1.

Set the clock time with 

```
sudo hwclock -w
```

Get the clock time with 

```
sudo hwclock -r
```

### Docker build

You need to build the docker containers before they can be used:

```
cd ~/security_system/
docker-compose build
```

### HostAPD Server 

The HostAPD wifi server is found under ./hostapd_docker. WIFI network ranges are already set up. DHCP is handled by the DHCP docker container.

You need to set up these two environment variables on the base system:
  - WPA2_ESSID
  - WPA2_PASSWORD

To build this container, I needed to do the following:

```
wget http://ftp.de.debian.org/debian/pool/main/libs/libseccomp/libseccomp2_2.5.1-1_armhf.deb
dpkg -i libseccomp2_2.5.1-1_armhf.deb
```

Once you copy this over to the system, you need to update the wpa2.conf file to fill in the variables:

```
envsubst < hostapd_docker/confs/hostapd_confs/wpa2_git.conf > hostapd_docker/confs/hostapd_confs/wpa2.conf
```

### DHCP Server

The dhcp configuration file is found under ./dhcp_server/data. The dhcpd.conf file was modified to support the (currently) three camera I have available based on the mac address:

| Camera Name | MAC Address | IP address |
| cam1 | f0-00-00-a3-0c-fe | 192.168.42.111 |
| cam2 | f0:00:00:a2:cf:7a | 192.168.42.112 |
| cam3 | f0:00:00:a2:df:9a | 192.168.42.113 |

The ip ranges are set up for the wlan0 network as well.

### Running DHCP and HostAPD with docker-compose

Run 

```
docker-compose up -d
```

This will restart the services on startup.

### Cameras

Each of the cameras is setup to use DHCP to get it's IP address.

To get information for setting up moonfire, I used the onvif-rs rust program.

```
git clone https://github.com/lumeohq/onvif-rs.git
cd onvif-rs
cargo run --example camera --  get-stream-uris --uri=http://192.168.42.11:8000 --username=<?> --password=<?>
```

Or use the print_cam_info.py to query to RTSP streams to use in the nvr setup.

Also set the system to use NTP to set up the clock. These cameras are limited in that they have a fixed number of NTP servers they can connect to. To get around this, set the servers hosts file to include the entry:

```
192.168.42.1  pool.ntp.org
```

This will force the camera to use the servers time.

### Setting Up Moonfire NVR

To start the configuration program:

```
sudo nvr config 2>debug-log
```

Create a new storage directory under /media/nvr/sample, and set up all your cameras appropriately. Following the moonfire-nvr instruction, set the flush_if_sec to 120.

To start the nvr docker container run 

```
sudo nvr run
```

This will restart the container every time the system starts up.

### Still to do:

Set up RTC, Setup server as NTP server for camera time stamping, set up display and button server, set up persistent capturing on power cycle (if needed, might already be in database.)

### Testing the OLED display

These tests are taken from [here](https://learn.adafruit.com/monochrome-oled-breakouts/python-usage-2)

```
sudo apt install libtiff-dev libopenjp2-7-dev
pip3 install adafruit-circuitpython-ssd1306 pillow
python test_screen.py
```

### Setting up a Local Docker Repository

Run a local registry:

```
docker run -d -p 5000:5000 --restart=always --name registry registry:2
```

You also need to set up the raspberry pi to allow unsecure downloads by editing the /etc/docker/daemon.json with:

```
{
  "insecure-registries" : ["myregistrydomain.com:5000"]
}
```

After building an image, you can tag and upload it to the local repository and then download it on the Raspberry Pi device.

```
docker build . -t <server_ip>:5000/my_image
docker push <server_ip>:5000/my_image
docker pull <server_ip>:5000/my_image
```

You can also use the docker save and load commands to create tar.gz images that can be copies manually.

### Building and Deploying Rust Hardware Control form x86

Instruction were found [here](https://medium.com/swlh/compiling-rust-for-raspberry-pi-arm-922b55dbb050).
