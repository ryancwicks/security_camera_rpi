from arm32v7/alpine:latest

# Install packages
RUN apk update && apk add hostapd iw iptables

# Configure Hostapd
ADD confs/hostapd_confs/wpa2.conf /etc/hostapd/hostapd.conf

# Configure networking
ADD confs/interfaces /etc/network/interfaces
ADD confs/iptables.sh /iptables.sh
ADD confs/iptables_off.sh /iptables_off.sh

# Copy and execute init file
ADD confs/start.sh /start.sh
CMD ["/bin/sh", "/start.sh"]