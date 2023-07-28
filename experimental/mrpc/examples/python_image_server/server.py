from python_image_server import ImageServer, ImageRequest, ImageResponse
from PIL import Image, ImageOps
from io import BytesIO


def grayscale_image(req: ImageRequest) -> ImageResponse:
    try:
        iob = BytesIO(req.data)
        im = Image.open(iob)
        out = ImageOps.grayscale(im)
        b = BytesIO()
        out.save(b,"png")
        byt = b.getvalue()
        return ImageResponse(byt)
    except Exception as e:
        print(e)
        return ImageResponse(b'oops')


s = ImageServer()
s.add_grayscale_handler(grayscale_image)
s.run("0.0.0.0:5000")