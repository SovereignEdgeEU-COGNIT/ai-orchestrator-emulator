import requests
import json
import random
from threading import Thread
import time

# Post address variable
POST_ADDRESS = "http://194.28.122.122:8000/start"
#POST_ADDRESS = "http://192.168.1.156:8000/start"

# Host IPs and names arrays
HOST_IPS = ["194.28.122.122", "194.28.122.123"]
HOST_NAMES = ["Cognit-test", "Cognit-test2"]

# Flavors and the number of instances to spawn
FLAVORS = ["cpu", "memory", "filesystem", "network"]
SPAWN_COUNT = [2, 2, 1, 1]

mem_range = range(256, 1537, 256)

random.seed(2)

# Function to send POST request
def send_post_request(ip, name, flavor):
    # Randomize cpu and mem
    CPU = round(random.uniform(0.2, 1.0), 1)
    MEM = random.choice(mem_range)
    
    # Randomize request_rate and periodicity
    execution_time = random.choice(range(3, 10, 2))
    request_rate = int(round(execution_time * random.uniform(1.5, 3)))
    
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
            # Sleep for 2 sec before starting the next thread
            time.sleep(2)

for thread in threads:
    thread.join()

        

            