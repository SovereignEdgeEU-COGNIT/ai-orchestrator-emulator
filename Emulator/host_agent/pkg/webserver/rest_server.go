package webserver

import (
	"bytes"
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

	fmt.Println("Received request to start container")

	if r.Method != http.MethodPost {
		http.Error(w, "Unsupported method", http.StatusMethodNotAllowed)
		fmt.Println("Unsupported method: ", r.Method)
		return
	}

	var req StartContainerRequest
	err := json.NewDecoder(r.Body).Decode(&req)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		fmt.Println("Error decoding request body: ", err.Error())
		return
	}

	fmt.Println("Starting container with CPU: ", req.CPU, " and Memory: ", req.Memory)
	_, containerName, err := dockerConfig.StartContainer(req.CPU, req.Memory)

	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		fmt.Println("Error starting container: ", err.Error())
		return
	}

	w.WriteHeader(http.StatusOK)
	json.NewEncoder(w).Encode(containerName)
}

// function to register the host agent with the controller
// curl $CTRL_PLANE_ADDR:$CTRL_PLANE_PORT/register/host -X POST -H "Content-Type: application/json" -d '{"ip": "*local-ip*", "name": "*host-name*" "port": "$HOST_AGENT_PORT"}'
func registerHostAgent(dockerConfig *docker.DockerConfig, port uint16) {
	url := fmt.Sprintf("http://%s:%s/register", dockerConfig.CtrlPlaneAddr, dockerConfig.CtrlPlanePort)
	payload := fmt.Sprintf(`{"Host":{"ip": "%s", "name": "%s", "port": %d}}`, dockerConfig.HostInfo.LocalIP, dockerConfig.HostInfo.Hostname, port)
	_, err := http.Post(url, "application/json", bytes.NewBuffer([]byte(payload)))
	if err != nil {
		log.Fatal(err)
	}
}

// function to launch the web server
func StartWebServer(dockerConfigNew *docker.DockerConfig, port uint16) {
	dockerConfig = dockerConfigNew
	registerHostAgent(dockerConfig, port)
	http.HandleFunc("/start-container", handleStartContainer)
	fmt.Println("WebServer is starting on port ", port, " ...")
	if err := http.ListenAndServe(fmt.Sprintf(":%d", port), nil); err != nil {
		log.Fatal(err)
	}
}
