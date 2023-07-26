from rpc_echo_python_server import Server

class MyServer(Server):
    def __init__(self, addr: str):
        self.addr = addr
    
    # def sayhello(self, req):
    #     print(req)
    #     print("python says hi!")


s = MyServer("0.0.0.0:5000")
s.run()