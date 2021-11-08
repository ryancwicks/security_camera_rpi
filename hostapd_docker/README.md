# HostAPD Docker Setup

These instruction are adapted from [here](https://fwhibbit.es/en/automatic-access-point-with-docker-and-raspberry-pi-zero-w).

## Setting up the host

The first step is to set up the operating system to allow for packet forwarding:

Edit the /etc/sysctl.conf and change the following:

```
# Uncomment the next line to enable packet forwarding for IPv4
net.ipv4.ip_forward=1
```

You also need to keep the dhcpd server from assigning an address to wlan0. Edit /etc/dhcpcd.conf

```
denyinterfaces wlan0
```
