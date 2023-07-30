# generate random color PNG file for testing
# use python3 generate_png.py <height> <width>

import sys
import random
from PIL import Image

if len(sys.argv) != 4:
    print('Usage: python3 generate_png.py <fname> <height> <width>.')

_, fname, height, width = sys.argv

if (not height.isdigit()) or (not width.isdigit()):
    print('Usage: python3 generate_png.py <fname> <height> <width>.')

height_i = int(height)
width_i = int(width)

pixels = [(random.randint(0, 255), 
           random.randint(0, 255), 
           random.randint(0, 255)
           ) for j in range(width_i) for i in range(height_i)]

img = Image.new("RGB", (width_i, height_i))
img.putdata(pixels)
img.save(fname, "PNG")