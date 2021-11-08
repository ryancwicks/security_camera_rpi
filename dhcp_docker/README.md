# Very Simple Local Interface DHCP server.

Use this to set up a local DHCP server so that you can communicate with or set up locally connected devices (Raspberry Pi's, ethenet cameras, etc).

Install the dhcps docker image:

docker pull networkbook/dhcpd

Setup you local interface (the one that will serve the dhcp requests) to the ip address 192.168.42.1/24.

Run the docker container from the current directory with 

```bash
docker run -it --rm --init --net host -v "$(pwd)/data":/data networkboot/dhcpd <Your interface> 
```

