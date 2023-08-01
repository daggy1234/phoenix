from rpc_echo_python_client import Connection, HelloRequest
import time
import csv
from random import random

c = Connection("172.31.27.226:5000")

for length in range(1,(10**9),10):
    time_l = []
    for i in range(1000):
        start = time.time()
        req_str = "".join(chr(random(0, 0x110000)) for _ in range(length))
        req = HelloRequest(f"test message number {i}")
        o = c.say_hello(req)
        end = time.time()
        diff = end - start
        time_l.append([str(i), diff])

    with open(f"./data/mrpc_data_{length}.csv", "w") as f:
        csvwriter = csv.writer(f)
        csvwriter.writerows(time_l)

# print(sum(time_l)/len(time_l))