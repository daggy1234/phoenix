from rpc_echo_python_client import Connection, HelloRequest
import time
import csv
from random import randbytes, randrange

c = Connection("172.31.27.226:5000")

for pow in range(7):
    length = 10 ** pow
    print(length)
    time_l = []
    for i in range(1000):
        req_str = "a" * length
        start = time.time()
        req = HelloRequest(req_str)
        o = c.say_hello(req)
        end = time.time()
        diff = end - start
        time_l.append([diff])

    with open(f"./data/mrpc_data_{length}.csv", "w") as f:
        csvwriter = csv.writer(f)
        csvwriter.writerows(time_l)

# print(sum(time_l)/len(time_l))