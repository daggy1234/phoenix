from rpc_echo_python_server import MyGreeter, HelloRequest, HelloResponse
import sys,inspect

def say_hello(req: HelloRequest) -> HelloResponse:
    msg = req.message
    return HelloResponse(f"Recieved {msg}!")



s = MyGreeter()
s.add_say_hello(say_hello)
s.run("0.0.0.0:5000")