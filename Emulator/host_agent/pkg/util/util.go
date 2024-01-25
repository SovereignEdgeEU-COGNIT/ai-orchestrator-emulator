package util

import (
	"fmt"
	"net"
	"os"
)

// GetHostname returns the hostname of the machine.
func getHostname() (string, error) {
	return os.Hostname()
}

// GetLocalIP returns the first non-loopback IP address found.
func getLocalIP() (string, error) {
	addrs, err := net.InterfaceAddrs()
	if err != nil {
		return "", err
	}
	for _, addr := range addrs {
		if ipnet, ok := addr.(*net.IPNet); ok && !ipnet.IP.IsLoopback() {
			if ipnet.IP.To4() != nil {
				return ipnet.IP.String(), nil
			}
		}
	}
	return "", fmt.Errorf("no non-loopback IP address found")
}

// Host struct
type Host struct {
	Hostname string
	LocalIP  string
}

// Returns the hostname and local ip of the host as a host struct
func GetHost() (Host, error) {
	hostname, err := getHostname()
	if err != nil {
		return Host{}, err
	}
	localIP, err := getLocalIP()
	if err != nil {
		return Host{}, err
	}
	return Host{hostname, localIP}, nil
}
