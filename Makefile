IP_ADDRESS=192.168.42.1

copy:
	rsync -avr -e "ssh -l pi" --exclude 'target/' --exclude 'venv/' ./* pi@${IP_ADDRESS}:/home/pi/security-system/
