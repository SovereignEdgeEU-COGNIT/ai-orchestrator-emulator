
# How to:
Loads data from the ../.env
1. builds the containers using docker compose (file ../docker-compose.yml)
2. if (local ip == CTRL_PLANE_ADDR) 
   1. start the prometheus, ctrl_plane and client_emulator containers
3. start the cAdvisor container
4. start the webserver 
   1. Webserver can start emulated_sr 
5. if SIGINT
   1. stop all containers

# Future work
Extend the host_agent to have a CLI for managing the host https://github.com/spf13/cobra
- [ ] docker containers & images
- [ ] web-server
- [ ] ...