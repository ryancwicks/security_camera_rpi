#!/bin/sh
sudo wpa_supplicant -Dnl80211 -iwlan0 -c/etc/wpa_supplicant/wpa_supplicant.conf &
sudo dhclient wlan0