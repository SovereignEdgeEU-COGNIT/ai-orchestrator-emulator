package webserver

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	"cognit/host_agent/pkg/docker"
)

var dockerConfig *docker.DockerConfig

type StartContainerRequest struct {
	CPU    float32 `json:"cpu"`
	Memory int     `json:"memory"`
}

func handleStartContainer(w http.ResponseWriter, r *http.Request) {

	if r.Method != http.MethodPost {
		http.Error(w, "Unsupported method", http.StatusMethodNotAllowed)
		return
	}

	var req StartContainerRequest
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	_, containerName, err := dockerConfig.StartContainer(req.CPU, req.Memory)

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(containerName)
}

// function to launch the web server
func StartWebServer(dockerConfigNew *docker.DockerConfig, port uint16) {
	dockerConfig = dockerConfigNew
	http.HandleFunc("/start-container", handleStartContainer)
	fmt.Println("WebServer is starting on port ", port, " ...")
	if err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil); err != nil {
		log.Fatal(err)
	}
}
