#!/bin/bash

# Log in and stop the currently running service
ssh rem "systemctl stop rm_text_share"

# Copy binary and service files to remarkable
scp target/armv7-unknown-linux-gnueabihf/release/rm_text_share rem:/usr/bin
scp rm_text_share.service rem:/etc/systemd/system

# Log in and start and enable the service
ssh rem "systemctl daemon-reload; systemctl start rm_text_share; systemctl enable rm_text_share"
