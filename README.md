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
```


### Hard Drive Setup

Create a ext4 partition on the large data hard drive (a 5TG USB3 drive in my case), first deleting the existing partition.

```
sudo fdisk /dev/sda #In my case
#delete existing partitions and create a new one that fills the drive.
mkfs.ext4 -T largefile -m 1 /dev/sdc1
```

Now create a mount point and set it up. First get the UUID for the USB drive:

```
sudo blkid
```

Now add this line to fstab, and follow the next steps to create and test the mount point.

```
sudo mkdir /media/nvr
sudo vi /etc/fstab
```

Add the following line to the bottom of the fstab file:

```
UUID=<Your UUID> /media/nvr ext4 nofail,noatime,lazytime,data=writeback,journal_async_commit  0  2
```

You should now be able to mount the /media/nvr directory.

### Adding the moonfire-nvr User

Create the moonfire-nvr user on the USB drive. This will prevent the SD card from being work out, and also allow the moonfire sql database to be created. 

'''
sudo useradd --user-group --create-home --home /var/lib/moonfire-nvr moonfire-nvr
'''


### Network Setup



### Docker build

You need to build the docker containers before they can be used:

```
cd ~/security_system/
docker-compose build
```

### HostAPD Server 

The HostAPD wifi server is found under ./hostapd_docker. WIFI network ranges are already set up. DHCP is handled by the DHCP docker container.

To build this container, I needed to do the following:

```
wget http://ftp.de.debian.org/debian/pool/main/libs/libseccomp/libseccomp2_2.5.1-1_armhf.deb
dpkg -i libseccomp2_2.5.1-1_armhf.deb
```

### DHCP Server

The dhcp configuration file is found under ./dhcp_server/data. The dhcpd.conf file was modified to support the (currently) three camera I have available based on the mac address:

| Camera Name | MAC Address | IP address |
| cam1 | f0-00-00-a3-0c-fe | 192.168.42.111 |
| cam2 | f0:00:00:a2:cf:7a | 192.168.42.112 |
| cam3 | f0:00:00:a2:df:9a | 192.168.42.113 |

The ip ranges are set up for the wlan0 network as well.

### Cameras

Each of the cameras is setup to use DHCP to get it's IP address.

