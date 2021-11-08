#!/usr/bin/env python3

import argparse
from onvif import ONVIFCamera


def PrintCam(ip, port, username, password):
    print(f'camera {ip}:{port}')
    c = ONVIFCamera(ip, port, username, password)

    c.create_devicemgmt_service()
    print(c.devicemgmt.GetDeviceInformation())

    c.create_media_service()
    for p in c.media.GetProfiles():
        resp = c.media.GetStreamUri({
            'StreamSetup': {'Stream': 'RTP-Unicast', 'Transport': {'Protocol': 'RTSP'}},
            'ProfileToken': p.token,
        })
        print(resp.Uri)


parser = argparse.ArgumentParser(description='Find RTSP URLs.')
parser.add_argument('--ip', dest='ip', required=True,
                   help='ip address (or hostname) of the camera')
parser.add_argument('--port', dest='port', type=int, default=80,
                   help='ONVIF port of the camera (default 80)')
parser.add_argument('--username', dest='username',
                   help='ONVIF administrator username')
parser.add_argument('--password', dest='password',
                   help='ONVIF administrator password')
args = parser.parse_args()
PrintCam(args.ip, args.port, args.username, args.password)