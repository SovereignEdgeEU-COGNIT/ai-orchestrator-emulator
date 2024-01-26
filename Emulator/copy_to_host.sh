#!/bin/bash

# Source and temporary destination directories
sourceFolder="./"
#tempDestinationFolder="../copy_temp"
tempDestinationFolder="../copy_temp"

# SSH details
sshHosts=("194.28.122.122" "194.28.122.123")
sshUser="root"
remoteDestination="/root/Emulator"

# Create temp destination folder
mkdir -p "$tempDestinationFolder"

# Copy files from source to temp destination, excluding 'target' subfolders
rsync -av --exclude '*/target/*' "$sourceFolder/" "$tempDestinationFolder/"

# Copy files from temp destination to remote SSH host
for host in "${sshHosts[@]}"; do
    echo "Copying files to $host"
    scp -r "$tempDestinationFolder"/* "$sshUser@$host:$remoteDestination"
done

# Clean up: remove temporary files
rm -rf "$tempDestinationFolder"
