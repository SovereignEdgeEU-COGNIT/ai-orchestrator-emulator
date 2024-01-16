#!/bin/bash

# Read arguments
CGROUP_NAME="$1"
HOST_URL="$2"
CMD_ARGS="$3"
EXECUTABLE="/path/to/executable"  # Replace with your executable path

# Add the current shell process to the specified cgroup
echo $$ > /sys/fs/cgroup/net_cls/${CGROUP_NAME}/tasks

# Function to send POST requests
send_post_request() {
    # Using curl to send POST request
    curl -X POST -H "Content-Type: application/json" -d "${CMD_ARGS}" "${HOST_URL}"
}

# Main loop
while true; do
    # Run the executable in the background (replace with actual arguments if needed)
    $EXECUTABLE &

    # Send the POST request
    send_post_request

    # Wait for 5 seconds
    sleep 5
done
