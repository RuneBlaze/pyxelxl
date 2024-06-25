import pyxel

from pyxelxl.font import Font
from pyxelxl.pyxelxl import LayoutOpts

font = Font("/Users/lbq/goof/genio/assets/DMSerifDisplay-Regular.ttf")
font2 = Font("/Users/lbq/goof/genio/assets/retro-pixel-petty-5h.ttf")
zh = Font("/Users/lbq/Downloads/zpix.ttf")


class App:
    def __init__(self):
        pyxel.init(320, 240)
        self.cam_x = 0
        self.cam_y = 0
        pyxel.run(self.update, self.draw)

    def update(self):
        pass
        self.cam_x = (self.cam_x + 1) % 320
        self.cam_y = (self.cam_y + 1) % 240
        pyxel.camera(self.cam_x - 100, self.cam_y - 100)

    def draw(self):
        pyxel.cls(9)
        img = font.rasterize("Hello, World!", 20, 255 // 2, 7, 0)
        pyxel.blt(0, 0, img, 0, 0, img.width, img.height, 0)
        font2.text(0, 20, "Hello, World!", 7, font_size=5)
        zh.text(
            0, 40, "我能吞下玻璃而不伤身体\n我能吞下玻璃而不伤身体", 7, font_size=12
        )
        pyxel.text(0, 70, "Hello, World!", 7)


App()
