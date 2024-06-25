import pyxel
from pyxelxl.font import Font

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
        roboto.draw(0, 0, "Hello, World! Antialiased", 7, font_size=16)
        zh_font.draw(0, 40, "我能吞下玻璃而不伤身体", 7, font_size=12)
        roboto.draw(0, 80, "Hello, World! Not antialiased", 15, font_size=16, threshold=128)

App()