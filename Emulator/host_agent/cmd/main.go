package main

import (
	"fmt"
	"log"
	"os"
	"os/signal"
	"strconv"

	"cognit/host_agent/pkg/docker"
	"cognit/host_agent/pkg/util"
	"cognit/host_agent/pkg/webserver"

	"github.com/joho/godotenv"
)

func main() {

	//load environment variables from ../.env
	err := godotenv.Load("../.env")
	if err != nil {
		log.Fatal("Error loading .env file")
	}

	imageName := os.Getenv("SR_IMAGE_NAME")
	portStr := os.Getenv("HOST_AGENT_PORT")

	if imageName == "" {
		log.Fatal("SR_IMAGE_NAME environment variable must be set")
	} else if portStr == "" {
		log.Fatal("HOST_AGENT_PORT environment variable must be set")
	}

	//port to uint16
	port, err := strconv.ParseUint(portStr, 10, 16)

	//err != nil and port less than uint16 max
	if err != nil && port < 65535 {
		log.Fatal("HOST_AGENT_PORT environment variable must be a valid port number")
	}

	hostInfo, err := util.GetHost()
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	dockerConfig := docker.NewDockerConfig(hostInfo, imageName)

	log.Default().Println("Docker initialized")

	go handleSigInt(dockerConfig)

	log.Default().Println("Starting web server...")
	webserver.StartWebServer(dockerConfig, uint16(port))

	// Add your logic to handle the allocatedPorts and containerIDName maps
}

// On ctrl+c run docker.StopAllContainers()
func handleSigInt(dockerConfig *docker.DockerConfig) {
	c := make(chan os.Signal, 1)
	signal.Notify(c, os.Interrupt)
	go func() {
		<-c
		dockerConfig.StopAllContainers()
		os.Exit(0)
	}()
}
