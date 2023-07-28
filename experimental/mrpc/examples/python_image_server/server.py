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

def invert_image(req: ImageRequest) -> ImageResponse:
    try:
        iob = BytesIO(req.data)
        im = Image.open(iob)
        out = ImageOps.invert(im)
        b = BytesIO()
        out.save(b,"png")
        byt = b.getvalue()
        return ImageResponse(byt)
    except Exception as e:
        print(e)
        return ImageResponse(b'oops')


def icon_image(req: ImageRequest) -> ImageResponse:
    try:
        iob = BytesIO(req.data)
        im = Image.open(iob)
        out = im.resize((32,32), resample=Image.Resampling.LANCZOS)
        b = BytesIO()
        out.save(b,"png")
        byt = b.getvalue()
        return ImageResponse(byt)
    except Exception as e:
        print(e)
        return ImageResponse(b'oops')

s = ImageServer()
s.add_grayscale_handler(grayscale_image)
s.add_invert_handler(invert_image)
s.add_icon_handler(icon_image)
s.run("0.0.0.0:5000")