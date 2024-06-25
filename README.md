# PyxelXL

> I am using this for my own game dev purposes, and this is beta-quality software.


"Bloat"-ish extensions for Pyxel, that might not fit the retro game-dev aesthetic, but fits more for the "let's throw up some constraints for our game jam so we can focus" aesthetic.

Fast TTF drawing for [Pyxel](https://github.com/kitao/pyxel), including support for layout (align, wrapping) and experimental support for anti-aliasing. This library is in the works to become a general purpose "bloated" set of extensions for Pyxel, but for now it only includes a font rendering extension and a center-rotation extension.

![alt screenshot](demo/pyxel-20240625-031732.gif)

## Installation

You can install PyxelXL using pip:

```bash
pip install pyxelxl
```

## Usage

To use a TTF font in your Pyxel application:

```python
import pyxel
from pyxelxl.font import Font
from pyxelxl import LayoutOpts

roboto = Font("/path/to/Roboto-Regular.ttf")
zh_font = Font("/path/to/zpix.ttf")

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
        ) # layout is optional. Thresholding makes the text look pixelated.

App()
```

### LayoutOpts, Text Wrapping, and Alignment

`LayoutOpts` is a class used to define layout options for text rendering in `pyxelxl`.

#### Attributes

- `max_width`: `Optional[int]`
  - The maximum width of the text layout in pixels. If `None`, the width is not constrained. Wrapping will occur if the text exceeds this width.

- `max_height`: `Optional[int]`
  - The maximum height of the text layout in pixels. If `None`, the height is not constrained.

- `horizontal_align`: `str`
  - The horizontal alignment of the text. It can be one of the following values:
    - `"left"`
    - `"center"`
    - `"right"`
  - Default is `"left"`.

- `vertical_align`: `str`
  - The vertical alignment of the text. It can be one of the following values:
    - `"top"`
    - `"center"`
    - `"bottom"`
  - Default is `"top"`.

- `line_height_mult`: `Optional[float]`
  - The multiplier for line height. If `None`, the line height is determined by the font size.

## Center Rotation

```python
from pyxelxl.rotate import blt
blt(80 - 32, 60 - 32, self.square, 0, 0, 64, 64, colkey=0, rot=self.angle)
# The rotation center is at (80, 60) in the global coordinate system.
```

## Advantages

 - Fast for font rendering
 - Easy multi-font size support
 - Anti-aliasing algorithm for non-pixel fonts

## Acknowledgements

Heavy-lifting done by [fontdue](https://github.com/mooman219/fontdue), and obviously this library will not
be here if not for Pyxel.