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

# Build the application in release mode
cargo build --release --bin "$APP_NAME"

# Copy the binary to /usr/local/bin
sudo cp "target/release/$APP_NAME" /usr/local/bin/

# Copy the systemd service file to the system directory
sudo cp "$APP_NAME/systemd/$SERVICE_NAME" /etc/systemd/system/

# Reload systemd manager configuration
sudo systemctl daemon-reload

# Enable the service to start on boot
sudo systemctl enable "$SERVICE_NAME"

# Start the service immediately
sudo systemctl start "$SERVICE_NAME"
