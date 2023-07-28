from python_image_client import Connection, ImageRequest
import time
import csv
from PIL import Image
from io import BytesIO

time_ls = []
with open("tree.jpg", "rb") as f:
    byt = f.read()
c = Connection("172.31.27.226:5000")
for i in range(1000):
    start_g = time.time()
    req_g = ImageRequest(byt)
    o = c.grayscale_image(req_g)
    end_g = time.time()


    start_i = time.time()
    req_i = ImageRequest(byt)
    o = c.invert_image(req_i)
    end_i = time.time()


    start_ic = time.time()
    req_ic = ImageRequest(byt)
    o = c.icon_image(req_ic)
    end_ic = time.time()

    time_ls.append([str(i), end_g - start_g, end_i - start_i, end_ic - start_ic])

with open("mrpc_data.csv", "w") as f:
    csvwriter = csv.writer(f)
    csvwriter.writerows(time_ls)

# print(sum(time_l)/len(time_l))