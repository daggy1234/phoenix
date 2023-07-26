from rpc_echo_python_client import Connection, HelloRequest
import time

c = Connection("0.0.0.0:5000")

for i in range(10):
    req = HelloRequest(f"test message number {i}")
    print("Sending Request")
    start = time.time()
    o = c.say_hello(req)
    end = time.time()
    diff = start - end
    print("Time=%s" % (diff))
    print(o.message)