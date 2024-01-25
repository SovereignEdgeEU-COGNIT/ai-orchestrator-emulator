#!/bin/bash

docker ps -a --format "{{.Names}}" | grep "^$(hostname)" | xargs -r docker stop
docker ps -a --format "{{.Names}}" | grep "^$(hostname)" | xargs -r docker rm

# Docker image name
IMAGE_NAME="emulator_emulated_host"

# Ports to map (host:container)
BASE_HOST_PORT=1234
BASE_CONTAINER_PORT=$BASE_HOST_PORT

# Get the hostname of the machine
MACHINE_HOSTNAME=$(hostname)

# Get the local IP address of the machine
LOCAL_IP=$(hostname -I | awk '{print $1}')

# Loop to create and start containers
for i in {1..4}
do
    CONTAINER_NAME="${MACHINE_HOSTNAME}_emulated_host_$i"
    HOST_NAME=$CONTAINER_NAME

    # Calculate ports for this container
    HOST_PORT=$((BASE_HOST_PORT + i))
    CONTAINER_PORT=$HOST_PORT

    # Environment variables
    ENV_VARS="-e CTRL_PLANE_ADDR=194.28.122.122 -e CTRL_PLANE_PORT=8000 -e HOST_IP=$LOCAL_IP -e HOST_PORT=$HOST_PORT"

    # Run the Docker container
    docker run -d \
        --name $CONTAINER_NAME \
        --hostname $HOST_NAME \
        -p "${HOST_PORT}:${CONTAINER_PORT}" \
        $ENV_VARS \
        $IMAGE_NAME
done
