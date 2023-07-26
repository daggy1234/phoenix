from rpc_echo_python_server import MyGreeter

class MyServer(MyGreeter):
    def __init__(self):
        print("init")
    
    def sayhello(self, req):
        print(req)
        print("python says hi!")


s = MyServer()
s.run("0.0.0.0:5000")