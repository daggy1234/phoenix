from rpc_echo_python_client import Connection, HelloRequest
import time
import csv

c = Connection("172.31.27.226:5000")


time_l = []
for i in range(3000):
    start = time.time()
    req = HelloRequest(f"test message number {i}")
    o = c.say_hello(req)
    end = time.time()
    diff = end - start
    time_l.append([str(i), diff])

with open("mrpc_data.csv", "w") as f:
    csvwriter = csv.writer(f)
    csvwriter.writerows(time_l)

# print(sum(time_l)/len(time_l))