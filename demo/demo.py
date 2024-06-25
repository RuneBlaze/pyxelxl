from pyxelxl import Font, FontDrawer
from PIL import Image
import pyxel
import numpy as np

with open('/Users/lbq/Downloads/Roboto/Roboto-Regular.ttf', 'rb') as fh:
    font = Font(fh.read())

rasterized = font.rasterize_text('Hello, World!', 14)
# print(rasterized.shape)


# Create boolean masks for each grade
grade1_mask = (rasterized < 2*255//4) & (rasterized >= 255//4)
grade2_mask = (rasterized < 3*255//4) & (rasterized >= 2*255//4)
grade3_mask = (rasterized >= 3*255//4)

def image_as_ndarray(image: pyxel.Image) -> np.ndarray:
    data_ptr = image.data_ptr()
    h, w = image.height, image.width
    return np.frombuffer(data_ptr, dtype=np.uint8).reshape(h, w)

class App:
    def __init__(self):
        pyxel.init(160, 120)
        self.drawer = FontDrawer(pyxel.colors.to_list())
        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pyxel.cls(0)
        arr = image_as_ndarray(pyxel.screen)
        self.drawer.imprint(rasterized, 7, 10, 10, arr)
        # grade = rasterized > 255//2
        # h, w = rasterized.shape
        # for i in range(h):
        #     for j in range(w):
        #         if grade[i, j] > 0:
        #             pyxel.pset(j, i, 7)

App()
# rasterized[rasterized < 255//4] = 0
# rasterized[(rasterized < 2*255//4) & (rasterized >= 255//4)] = 85
# rasterized[(rasterized < 3*255//4) & (rasterized >= 2*255//4)] = 170
# rasterized[3*255//4 <= rasterized] = 255
# image = Image.fromarray(rasterized)

# # Save the image
# image.save('output_image.png')
# # zoom in

# # Resize the image
# image = image.resize((image.width * 10, image.height * 10), resample=Image.NEAREST)

# # Optionally, show the image
# image.show()