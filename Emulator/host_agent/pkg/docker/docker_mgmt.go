package docker

import (
	"bytes"
	"fmt"
	"log"
	"os"
	"os/exec"
	"strconv"
	"strings"
	"sync"

	"cognit/host_agent/pkg/util"
)

type DockerContainerConfig struct {
	ImageName     string
	ContainerName string
	HostName      string
	LocalIP       string
	HostPort      int
	CPU           float32
	Memory        int
}

type DockerConfig struct {
	CtrlPlaneAddr  string
	CtrlPlanePort  string
	startPort      uint16
	allocatedPorts map[uint16]bool
	HostInfo       util.Host
	contIDNameMap  map[string]string
	portMutex      sync.Mutex
	imageName      string
	prometheus     string
	ctrlPlane      string
	clientEmulator string
	monitoring     string
}

// Initialize the Docker config with default values
func NewDockerConfig(host util.Host, imageName string) *DockerConfig {
	config := &DockerConfig{
		CtrlPlaneAddr:  os.Getenv("CTRL_PLANE_ADDR"),
		CtrlPlanePort:  os.Getenv("CTRL_PLANE_PORT"),
		startPort:      1024,
		allocatedPorts: make(map[uint16]bool),
		HostInfo:       host,
		contIDNameMap:  make(map[string]string),
		portMutex:      sync.Mutex{},
		imageName:      imageName,
		prometheus:     "",
		ctrlPlane:      "",
		clientEmulator: "",
		monitoring:     "",
	}

	if config.CtrlPlaneAddr == "" || config.CtrlPlanePort == "" {
		log.Fatal("CTRL_PLANE_ADDR and CTRL_PLANE_PORT environment variables must be set")
	}

	log.Default().Println("Building images...")
	if output, err := config.buildImages(); err != nil {
		log.Fatal("Couldn't build images, aborting:", err.Error(), " output: ", output)
	}
	log.Default().Println("Images built successfully")

	if config.HostInfo.LocalIP == config.CtrlPlaneAddr {
		log.Default().Println("Starting Prometheus...")
		if output, err := config.startPrometheus(); err != nil {
			config.StopAllContainers()
			log.Fatal("Couldn't start Prometheus, aborting:", err.Error(), " output: ", output)
		} else {
			config.prometheus = output
			log.Default().Println("Prometheus started successfully, ID: ", output)
		}

		log.Default().Println("Starting control plane...")
		if output, err := config.startCtrlPlane(); err != nil {
			config.StopAllContainers()
			log.Fatal("Couldn't start control plane, aborting:", err.Error(), " output: ", output)
		} else {
			config.ctrlPlane = output
			log.Default().Println("Control plane started successfully, ID: ", output)
		}

		log.Default().Println("Starting client emulator...")
		if output, err := config.startClientEmulator(); err != nil {
			config.StopAllContainers()
			log.Fatal("Couldn't start client emulator, aborting:", err.Error(), " output: ", output)
		} else {
			config.clientEmulator = output
			log.Default().Println("Client emulator started successfully, ID: ", output)
		}
	}

	log.Default().Println("Starting monitoring container...")
	if output, err := config.startMonitoringContainer(); err != nil {
		config.StopAllContainers()
		log.Fatal("Couldn't start monitoring container, aborting:", err.Error(), " output: ", output)
	} else {
		config.monitoring = output
		log.Default().Println("Monitoring container started successfully, ID: ", output)
	}

	return config
}

// Initialize the Docker container config with default values
/* func NewDockerContainerConfig() *DockerContainerConfig {
	return &DockerContainerConfig{
		ImageName:     "",
		ContainerName: "",
		HostName:      "",
		LocalIP:       "",
		HostPort:      0,
		CPU:           0,
		Memory:        0,
	}
}
*/
// allocatePort finds an available port
func (dc *DockerConfig) allocatePort() uint16 {
	dc.portMutex.Lock()
	defer dc.portMutex.Unlock()
	for port := dc.startPort; port <= 65535; port++ {
		if _, used := dc.allocatedPorts[port]; !used {
			dc.allocatedPorts[port] = true
			return port
		}
	}
	return 0 // Indicates no available port, handle this case in your code
}

func startContainer(cmdArgs []string) (string, error) {

	//print the command to be executed
	log.Default().Println("Executing command: ", "docker", cmdArgs)

	cmd := exec.Command("docker", cmdArgs...)
	//print the args to make sure they are in the right order
	log.Default().Println("Command args: ", cmd.Args)
	var outb, errb bytes.Buffer
	cmd.Stdout = &outb
	cmd.Stderr = &errb
	err := cmd.Run()

	if err != nil {
		return errb.String(), err
	} else {
		//containerID := strings.TrimSpace(outb.String())
		return outb.String(), nil
	}
}

// StartContainer starts a Docker container and returns its ID.
/* func StartContainer(imageName, containerName, hostName, localIP string, hostPort int) (string, error) {
	containerPort := hostPort
	envVars := fmt.Sprintf("-e CTRL_PLANE_ADDR=%s -e CTRL_PLANE_PORT=%s -e HOST_IP=%s -e HOST_PORT=%d", ctrlPlaneAddr, ctrlPlanePort, localIP, hostPort)
	cmd := exec.Command("docker", "run", "-d", "--name", containerName, "--hostname", hostName, "-p", fmt.Sprintf("%d:%d", hostPort, containerPort), envVars, imageName)
	output, err := cmd.CombinedOutput()
	if err != nil {
		return "", err
	}
	containerID := strings.TrimSpace(string(output))
	containerIDName[containerID] = containerName
	return containerID, nil
} */

// StartContainer starts a Docker container with the specified configuration
func (dc *DockerConfig) StartContainer(cpu float32, memory int) (string, string, error) {
	hostPort := dc.allocatePort()
	if hostPort == 0 {
		return "", "", fmt.Errorf("no available port found")
	}

	containerPort := hostPort

	//envVars := fmt.Sprintf("-e CTRL_PLANE_ADDR=\"%s\" -e CTRL_PLANE_PORT=\"%s\" -e HOST_IP=\"%s\" -e HOST_PORT=\"%d\"", dc.CtrlPlaneAddr, dc.CtrlPlanePort, dc.HostInfo.LocalIP, hostPort)
	//envVars but as arrays of "-e" and "key=value" pairs with values formated using the dc struct
	envVars := []string{"-e", "CTRL_PLANE_ADDR=" + dc.CtrlPlaneAddr, "-e", "CTRL_PLANE_PORT=" + dc.CtrlPlanePort, "-e", "HOST_IP=" + dc.HostInfo.LocalIP, "-e", "HOST_PORT=" + strconv.Itoa(int(hostPort))}
	containerName := fmt.Sprintf("%s_%s_%d", dc.HostInfo.Hostname, "emu_sr", hostPort)

	cmdArgs := []string{"run", "--detach=true", "--name", containerName, "--hostname", containerName}
	if cpu > 0 {
		cmdArgs = append(cmdArgs, "--cpus", fmt.Sprintf("%.2f", cpu))
	}
	if memory > 0 {
		cmdArgs = append(cmdArgs, "--memory", fmt.Sprintf("%dm", memory))
	}
	cmdArgs = append(cmdArgs, "-p", fmt.Sprintf("%d:%d", hostPort, containerPort))
	cmdArgs = append(cmdArgs, envVars...)
	cmdArgs = append(cmdArgs, dc.imageName)

	output, err := startContainer(cmdArgs)

	//output, err := dc.startSrContainer(cpu, memory, hostPort)

	if err != nil {
		fmt.Println("Error starting container: ", err.Error(), " output: ", output)
		return "", "", err
	} else {
		containerID := strings.TrimSpace(output)
		fmt.Println("Container started successfully, ID: ", containerID)
		dc.contIDNameMap[containerID] = containerName
		return containerID, containerName, err
	}
}

func (dc *DockerConfig) startSrContainer(cpu float32, memory int, hostPort uint16) (string, error) {
	/* envArgs := []string{"run",
	"--detach=true",
	"--name",
	"test_name_1",
	"--hostname",
	"test_name_1",
	"--cpus", "0.5",
	"--memory", "512m",
	"-p", "1236:1236",
	"-e", "CTRL_PLANE_ADDR=194.28.122.122",
	"-e", "CTRL_PLANE_PORT=8000",
	"-e", "HOST_IP=194.28.122.122",
	"-e", "HOST_PORT=1236",
	"emulator-emulated_sr"} */

	envVars := []string{
		"-e", "CTRL_PLANE_ADDR=194.28.122.122",
		"-e", "CTRL_PLANE_PORT=8000",
		"-e", "HOST_IP=194.28.122.122",
		"-e", "HOST_PORT=1236",
	}

	containerName := fmt.Sprintf("%s_%s_%d", dc.HostInfo.Hostname, "emu_sr", len(dc.contIDNameMap))

	cmdArgs := []string{"run", "--detach=true", "--name", containerName, "--hostname", containerName}
	if cpu > 0 {
		cmdArgs = append(cmdArgs, "--cpus", fmt.Sprintf("%.2f", cpu))
	}
	if memory > 0 {
		cmdArgs = append(cmdArgs, "--memory", fmt.Sprintf("%dm", memory))
	}
	containerPort := hostPort
	cmdArgs = append(cmdArgs, "-p", fmt.Sprintf("%d:%d", hostPort, containerPort), "--rm")
	//add envVars to cmdArgs
	cmdArgs = append(cmdArgs, envVars...)
	cmdArgs = append(cmdArgs, dc.imageName)

	return startContainer(cmdArgs)

}

func (dc *DockerConfig) startMonitoringContainer() (string, error) {

	// Start cAdvisor using the following command:
	cmdArgs := []string{"run",
		"--volume=/:/rootfs:ro",
		"--volume=/var/run:/var/run:rw",
		"--volume=/sys:/sys:ro",
		"--volume=/var/lib/docker/:/var/lib/docker:ro",
		"--volume=/dev/disk/:/dev/disk:ro",
		"--publish=8080:8080",
		"--detach=true",
		"--name=cadvisor",
		"--rm",
		"emulator-cadvisor"}

	return startContainer(cmdArgs)
}

func (dc *DockerConfig) startPrometheus() (string, error) {

	//get absolute path for ../monitor/Prometheus
	absPath, err := os.Getwd()
	if err != nil {
		log.Fatal(err)
	}
	absPath = absPath[:strings.LastIndex(absPath, "/")]
	absPath = absPath + "/monitor/Prometheus"

	cmdArgs := []string{"run",
		//mount volume of absPath
		"--volume=" + absPath + ":/etc/prometheus",
		"--publish=9090:9090",
		"--detach=true",
		"--name=prometheus",
		"--rm",
		"emulator-prometheus"}

	return startContainer(cmdArgs)
}

func (dc *DockerConfig) startCtrlPlane() (string, error) {
	cmdArgs := []string{"run",
		"--publish=8000:8000",
		"--detach=true",
		"--name=ctrl_plane",
		"--hostname=ctrl_plane",
		"--env-file=../.env",
		"--rm",
		"emulator-ctrl_plane"}

	return startContainer(cmdArgs)
}

func (dc *DockerConfig) startClientEmulator() (string, error) {

	cmdArgs := []string{"run",
		"--detach=true",
		"--name=client_emulator",
		"--hostname=client_emulator",
		"--env-file=../.env",
		"--rm",
		"emulator-client_emulator"}

	return startContainer(cmdArgs)
}

func (dc *DockerConfig) buildImages() (string, error) {

	//specify the path to the docker-compose.yml file
	cmdArgs := []string{"compose",
		"-f", "../docker-compose.yml",
		"build"}

	cmd := exec.Command("docker", cmdArgs...)
	var outb, errb bytes.Buffer
	cmd.Stdout = &outb
	cmd.Stderr = &errb
	err := cmd.Run()

	if err != nil {
		return errb.String(), err
	} else {
		return outb.String(), nil
	}
}

func (dc *DockerConfig) StopAllContainers() {

	cmdArgs := []string{"stop"}

	if dc.prometheus != "" {
		cmdArgs = append(cmdArgs, dc.prometheus)
	}
	if dc.ctrlPlane != "" {
		cmdArgs = append(cmdArgs, dc.ctrlPlane)
	}
	if dc.clientEmulator != "" {
		cmdArgs = append(cmdArgs, dc.clientEmulator)
	}
	if dc.monitoring != "" {
		cmdArgs = append(cmdArgs, dc.monitoring)
	}

	// for each id in contIDNameMap add it to the cmdArgs
	for id := range dc.contIDNameMap {
		cmdArgs = append(cmdArgs, id)
	}

	log.Default().Println("Stopping containers... ", cmdArgs)

	cmd := exec.Command("docker", cmdArgs...)
	var outb, errb bytes.Buffer
	cmd.Stdout = &outb
	cmd.Stderr = &errb
	err := cmd.Run()

	if err != nil {
		log.Default().Println("Couldn't stop containers, aborting:", err.Error(), " output: ", errb.String())

	} else {
		log.Default().Println("Containers stopped successfully")
	}

}

func (dc *DockerConfig) cleanUp() (string, error) {

	cmdArgs := []string{"system", "prune", "-f"}

	cmd := exec.Command("docker", cmdArgs...)
	var outb, errb bytes.Buffer
	cmd.Stdout = &outb
	cmd.Stderr = &errb
	err := cmd.Run()

	if err != nil {
		return errb.String(), err
	} else {
		return outb.String(), nil
	}
}
