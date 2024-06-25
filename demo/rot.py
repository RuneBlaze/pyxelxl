import math

import pyxel

from pyxelxl import Font, LayoutOpts
from pyxelxl import blt_rot as blt

zh_font = Font("/Users/lbq/Downloads/zpix.ttf")
zh_font_text = zh_font.specialize(font_size=12)


class App:
    def __init__(self):
        pyxel.init(160, 120, title="PyxelXL Example")
        self.square = pyxel.Image(64, 64)
        self.square.rect(0, 0, 32, 32, 7)
        self.square.rect(32, 32, 64, 64, 7)
        self.square.rect(0, 32, 32, 64, 4)
        self.square.rect(32, 0, 64, 32, 3)
        self.square.circ(20, 20, 10, 8)
        self.angle = 0
        self.timer = 0
        pyxel.run(self.update, self.draw)

    def update(self):
        self.angle += abs(math.sin((self.timer * 0.2) * 4))
        self.timer += 1

    def draw(self):
        pyxel.cls(1)
        zh_font_text(
            0,
            40,
            "我能吞下玻璃而伤身体 " * 5,
            7,
            layout=LayoutOpts(max_width=160, horizontal_align="center"),
        )
        blt(80 - 32, 60 - 32, self.square, 0, 0, 64, 64, colkey=0, rot=self.angle)


App()
