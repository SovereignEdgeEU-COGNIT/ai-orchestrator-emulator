
# How to:
## Install host for emulation
chmod +x ./install_for_host.sh
./install_for_host.sh

## Note for Simon
Make the Images smaller:
https://stackoverflow.com/questions/69144154/why-is-the-rust-docker-image-so-huge

## Endpoints
curl 194.28.122.122:8000/list?node_type=host
curl 194.28.122.122:8000/list?node_type=sr
curl 194.28.122.122:8000/register -X POST -H "Content-Type: application/json" -d '{"SR":{"ip": "194.28.122.123", "name": "Cognit-test-2_emu_sr_cpu_1025", "port": 1025, "client_info":{"flavor":"cpu", "execution_time":10, "request_rate":15}, "host_info":{"ip":"194.28.122.123","name":"Cognit-test-2","port":8001}}}'
curl http://194.28.122.122:8000/start -X POST -H "Content-Type: application/json" -d '{"host_info": {"ip":"194.28.122.122","name":"Cognit-test","port":8001}, "flavor": "cpu", "sr_env": {"cpu": 1.0, "mem": 1024}}'

 docker run -it --name Cognit-test_emu_sr_2 --hostname Cognit-test_emu_sr_2 --cpus 1.00 --memory 1024m -p 1024:1024 -e CTRL_PLANE_ADDR=194.28.122.122 -e CTRL_PLANE_PORT=8000 -e HOST_IP=194.28.122.122 -e HOST_PORT=1024 emulator-emulated_sr /bin/bash
 
docker kill Cognit-test_emu_sr_100
docker rm Cognit-test_emu_sr_100
docker run --detach=true --name Cognit-test_emu_sr_100 --hostname Cognit-test_emu_sr_100 --cpus 1.00 --memory 1024m -p 1025:1025 -e CTRL_PLANE_ADDR=194.28.122.122 -e CTRL_PLANE_PORT=8000 -e HOST_IP="194.28.122.122" -e HOST_PORT="1025" emulator-emulated_sr


curl http://$CTRL_PLANE_ADDR:$CTRL_PLANE_PORT/register/host -X POST -H "Content-Type: application/json" -d '{"Host": {"ip": "194.28.122.122", "name": "tired-simon", "port": '$HOST_AGENT_PORT'}}'

## Docker

docker system prune -a
docker image rm $(docker image ls -q)
docker container rm $(docker container ls -q)
docker volume rm $(docker volume ls -q)
docker build -f path_to_dockerfile -t docker_image_name .
docker run -it image_id/image_name /bin/bash

# Old description:
Caveats
This is mostly for @Johan Kristiansson but for anyone else interested:
I’m writing a short description of how to manage the current emulator. 
I know this is far from perfect but testing integration is prio 1 so improvements into mgmt comes later
There are many minute details still unspecified but this env. Is changing rapidly so no need to over-specify until it has crystalized a bit. 


INFO and TERMINOLOGY
First, we’re currently running 2 hardware hosts (this is 2 VMs provided by Johan), 194.28.122.122 & 194.28.122.123 – we call these “HOSTS”
We also have “VMs”. These are the machines running Serverless runtimes (SRs) in Cognit, currently a 1-to-1 mapping, thus VM = SR. 
In the emulation these are Docker containers running on our HOSTS.
Sadly these are currently named “Emulated_hosts” in the emulation, this was due to a shift in architecture, I’ll change this soon to be “Emulated_SR” soon and thus I’ll call it as such henceforth. 
The Emulated_SR can run a functions/job, and a job have a “flavor”.  Meaning we map Emulated_SR to flavor in a many-to-one mapping (Emulated_SR runs one flavor, but a flavor can be on many Emulated_SRs)
We’re also running Prometheus to gather the metrics and cAdvisor to provide the metrics.

STRUCTURE
Repo: https://github.com/SovereignEdgeEU-COGNIT/ai-orchestrator-emulator
Prometheus: ./Emulator/monitor/Prometheus
                           1 instance, running on .122
cAdvisor: ./Emulator/monitor/cAdvisor
                           1 instance per HOST
Ctrl_plane: ./Emulator/ctrl_plane
                           1 instance, running on .122
Emulated_client: ./Emulator/emulated_client
                           1 instance, running on .122
Emulated_SR: ./Emulator/emulated_host (or later emulated_sr)
                           Multiple instances per HOST

MANAGEMENT
Start system: ssh root@194.28.122.122 -> cd ./Emulator -> docker-compose up
Start SRs (later automatic when starting a new function): ssh root@194.28.122.122 / root@194.28.122.123 -> cd ./Emulator -> ./bash_for_hosts.sh (this also shutsdown any running hosts, comment out line 3-4 to disable that)
Get SR: curl -s http://194.28.122.122:8000/hosts (will update to /SR later) 
Get SR-Flavor mapping: curl -s http://194.28.122.122:8000/hosts/flavor
Start a function at a SR: a curl -s -X POST "http://194.28.122.122:8000/start" -H "Content-Type: application/json" -d "{\"host\":{\"ip\":\"194.28.122.123\",\"name\":\"Cognit-test-2_emulated_host_3\",\"port\":1237}, \"flavors\": [\"cpu\"]}"
                           In this JSON, feel free to change what’s within “host” and “flavors”.
                           Host change into one of the JSONs returned by /hosts
                           Flavor can be either “cpu” or “disk” at the moment.
                           I’ll wrap this into a neat function later, that won’t need anything other than “flavor” info, but that’s a later prio





rate(container_cpu_user_seconds_total[10s])

#docker stop $(docker ps -a -q)
docker ps -a --format "{{.Names}}" | grep "^$(hostname)" | xargs -r docker stop
docker ps -a --format "{{.Names}}" | grep "^$(hostname)" | xargs -r docker rm

docker ps -a
docker logs 2e5a5cb4f907
docker exec -it 2e5a5cb4f907 sh

curl -s "http://194.28.122.122:8000/hosts"
curl -s "http://194.28.122.122:8000/hosts/flavor"

curl -s -X POST "http://194.28.122.122:8000/start" -H "Content-Type: application/json" -d "{\"host\":{\"ip\":\"194.28.122.123\",\"name\":\"Cognit-test-2_emulated_host_3\",\"port\":1237}, \"flavors\": [\"cpu\"]}"


alias docker_stop='docker kill (docker ps -q)'
alias docker_remove_all_containers='docker rm (docker ps -a -q)'
alias docker_remove_all_images='docker rmi (docker images -q)'
alias docker_remove_all_volumes='docker volume rm (docker volume ls -q)'
docker-compose down --volume