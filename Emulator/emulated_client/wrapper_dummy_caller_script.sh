#!/bin/bash

# Read arguments
CGROUP_NAME="$1"
CLASS_ID="$2"
HOST_URL="$3"
CMD_ARGS="$4"
TC_MODIFY_SCRIPT="./tc_modify_script.sh"
#EXECUTABLE="/path/to/executable"  # Replace with your executable path


CGROUP_DIR="/sys/fs/cgroup/net_cls/${CGROUP_NAME}"


# Add the current shell process to the specified cgroup
#echo $$ > /sys/fs/cgroup/net_cls/${CGROUP_NAME}/tasks


# Add the current shell process to the specified cgroup
echo $$ > ${CGROUP_DIR}/tasks


# Set up iptables rule to mark packets from this cgroup
# Replace 0x10001 with the actual classid assigned to this cgroup
iptables -t mangle -A OUTPUT -m cgroup --cgroup $CLASS_ID -j MARK --set-mark $CLASS_ID

# Call the TC modification script
$TC_MODIFY_SCRIPT

# Function to send POST requests
send_post_request() {
    # Using curl to send POST request
    curl -X POST -H "Content-Type: application/json" -d "${CMD_ARGS}" "${HOST_URL}"
}

# Main loop
#while true; do
    # Run the executable in the background (replace with actual arguments if needed)
    #$EXECUTABLE &

    # Send the POST request
send_post_request

    # Wait for 5 seconds
    #sleep 5
#done
