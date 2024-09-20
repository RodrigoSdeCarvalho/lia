#!/bin/bash

sudo mkdir -p /usr/local/bin/lia-src
sudo mkdir -p /usr/local/bin/lia-src/logs

sudo cp ./target/release/cli /usr/local/bin/lia

sudo cp ./.env /usr/local/bin/lia-src/.env
sudo cp ./configs.json /usr/local/bin/lia-src/configs.json

if ! [ -x "$(command -v docker)" ]; then
  echo "Docker is not installed. Please install Docker before proceeding."
  exit 1
fi

echo "Installation complete. Run 'lia init' to initialize."