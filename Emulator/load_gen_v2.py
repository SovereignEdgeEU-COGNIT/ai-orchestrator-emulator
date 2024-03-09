import requests
import json
import random
from threading import Thread

# Post address variable
POST_ADDRESS = "http://194.28.122.122:8000/start"

# Host IPs and names arrays
HOST_IPS = ["194.28.122.122", "194.28.122.123"]
HOST_NAMES = ["Cognit-test", "Cognit-test2"]

# Flavors and the number of instances to spawn
FLAVORS = ["cpu", "memory", "io", "network"]
SPAWN_COUNT = [3, 3, 2, 2]

mem_range = range(256, 2049, 256)

random.seed(0)

# Function to send POST request
def send_post_request(ip, name, flavor):
    # Randomize cpu and mem
    CPU = round(random.uniform(0.2, 1.5), 1)
    MEM = random.choice(mem_range)
    
    # Randomize request_rate and periodicity
    execution_time = random.choice(range(3, 40, 2))
    request_rate = int(round(execution_time * random.uniform(1.2, 3)))
    
    data = {
        "host_info": {"ip": ip, "name": name, "port": 8001},
        "client_info": {
            "flavor": flavor,
            "execution_time": execution_time,
            "request_rate": request_rate
        },
        "sr_env": {"cpu": CPU, "mem": MEM}
    }
    headers = {'Content-Type': 'application/json'}
    #print(data)
    response = requests.post(POST_ADDRESS, headers=headers, data=json.dumps(data))
    print(response.text)
    # Save the data to a file
    with open('load_gen_data.txt', 'a') as file:
        file.write(json.dumps(data) + '\n')

threads = []
# Main loop to call the function for each host and flavor
for i in range(len(HOST_IPS)):
    for j in range(len(FLAVORS)):
        for k in range(SPAWN_COUNT[j]):

            thread = Thread(target=send_post_request, args=(HOST_IPS[i], HOST_NAMES[i], FLAVORS[j]))
            threads.append(thread)
            thread.start()

for thread in threads:
    thread.join()

        

            