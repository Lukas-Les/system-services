#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

APP_NAME="$1"
SERVICE_NAME="$APP_NAME.service"

# Check if APP_NAME was provided
if [ -z "$APP_NAME" ]; then
    echo "Usage: $0 <app_name>"
    exit 1
fi


sudo systemctl stop "$SERVICE_NAME"
sudo systemctl disable "$SERVICE_NAME"
sudo rm "/usr/local/bin/$APP_NAME"
sudo rm "/etc/systemd/system/$SERVICE_NAME"
sudo systemctl daemon-reload
sudo systemctl reset-failed
