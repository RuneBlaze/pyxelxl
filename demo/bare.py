import pyxel

from pyxelxl.font import Font
from pyxelxl.pyxelxl import LayoutOpts

roboto = Font("/Users/lbq/Downloads/Roboto/Roboto-Regular.ttf")
zh_font = Font("/Users/lbq/Downloads/zpix.ttf")


class App:
    def __init__(self):
        pyxel.init(160, 120, title="PyxelXL Example")
        pyxel.run(self.update, self.draw)

    def update(self):
        pass

    def draw(self):
        pyxel.cls(1)
        roboto.text(
            0,
            0,
            "Hello, World! Antialiased",
            7,
            font_size=16,
            layout=LayoutOpts(max_width=160, horizontal_align="center"),
        )
        zh_font.text(
            0,
            40,
            "我能吞下玻璃而不伤身体" * 5,
            7,
            font_size=12,
            layout=LayoutOpts(max_width=160),
        )
        roboto.text(
            0, 80, "Hello, World! Not antialiased", 15, font_size=16, threshold=128
        )


App()
