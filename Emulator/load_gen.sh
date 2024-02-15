#!/bin/bash

# Post address variable
POST_ADDRESS="194.28.122.122:8000/start"

# Host IPs and names arrays
HOST_IPS=("194.28.122.122" "194.28.122.123")
HOST_NAMES=("Cognit-test" "Cognit-test2")

# Flavors and the number of instances to spawn
FLAVORS=("cpu" "mem" "io" "net")
SPAWN_COUNT=(2 2 1 1)

# Setting cpu and mem
CPU="1"
MEM="512"

# Function to send POST request
send_post_request() {
    local ip=$1
    local name=$2
    local flavor=$3

    curl $POST_ADDRESS -X POST -H "Content-Type: application/json" -d "{
        \"host_info\": {\"ip\":\"$ip\",\"name\":\"$name\",\"port\":8001},
        \"flavor\": \"$flavor\",
        \"sr_env\": {\"cpu\": $CPU, \"mem\": $MEM}
    }"
}

# Main loop to call the function for each host and flavor
for i in "${!HOST_IPS[@]}"; do
    for j in "${!FLAVORS[@]}"; do
        for ((k=0; k<${SPAWN_COUNT[$j]}; k++)); do
            send_post_request "${HOST_IPS[$i]}" "${HOST_NAMES[$i]}" "${FLAVORS[$j]}"
        done
    done
done
