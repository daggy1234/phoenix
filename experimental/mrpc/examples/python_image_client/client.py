from python_image_client import Connection, ImageRequest
import time
import csv
from PIL import Image
from io import BytesIO

c = Connection("0.0.0.0:5000")
with open("tree.jpg", "rb") as f:
    byt = f.read()
start = time.time()
req = ImageRequest(byt)
o = c.icon_image(req)
bio = BytesIO(o.data)
im = Image.open(bio)
im.save("icon.png")
# with open("grayscale.png", "wb") as f:
#     f.write(o.data)
end = time.time()
diff = end - start
print(diff)

# print(sum(time_l)/len(time_l))