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

ENV WPA2_ESSID $WPA2_ESSID
ENV WPA2_PASSWORD $WPA2_PASSWORD

#RUN sed -i -s "s/^ssid=.*/ssid=${WPA2_ESSID}/" /etc/hostapd/hostapd.conf
#RUN sed -i -s "s/^password=.*/password=${WPA2_PASSWORD}/" /etc/hostapd/hostapd.conf


CMD ["/bin/sh", "/start.sh"]