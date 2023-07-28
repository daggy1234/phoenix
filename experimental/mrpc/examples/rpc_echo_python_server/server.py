from rpc_echo_python_server import MyGreeter, HelloRequest, HelloResponse


def say_hello(req: HelloRequest) -> HelloResponse:
    try:
        msg = req.message
        print(f"Got {msg}")
        return HelloResponse(f"Recieved {msg}!")
    except Exception as e:
        print(e)
        return HelloResponse(b'hiii')


s = MyGreeter()
s.add_say_hello(say_hello)
s.run("0.0.0.0:5000")