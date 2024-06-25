import math
from typing import Optional, Union

from pyxel import Image
from pyxel import blt as _blt

from pyxelxl.pyxelxl import rotate

from .font import _image_as_ndarray


def blt_centered(
    x: int,
    y: int,
    img: Union[int, Image],
    u: int,
    v: int,
    w: int,
    h: int,
    colkey: int,
    rot: float = 0.0,
) -> None:
    # Create a buffer image and copy the relevant part of the original image to it
    buffer = Image(w, h)
    buffer.blt(0, 0, img, u, v, w, h)

    # Convert the buffer image to an ndarray and apply rotation
    buffer_arr = _image_as_ndarray(buffer)
    new_buffer = rotate(buffer_arr, colkey, rot)

    nh, nw = new_buffer.shape
    new_image = Image(nw, nh)
    _image_as_ndarray(new_image)[:] = new_buffer

    # Calculate the new center of the rotated image
    new_cx, new_cy = nw // 2, nh // 2
    tx_rot = 0
    ty_rot = 0

    # Translate back from origin to new center
    nx = x + tx_rot - new_cx
    ny = y + ty_rot - new_cy

    _blt(int(nx), int(ny), new_image, 0, 0, nw, nh, colkey=colkey)


def blt(
    x: int,
    y: int,
    img: Union[int, Image],
    u: int,
    v: int,
    w: int,
    h: int,
    colkey: int,
    rot: float = 0.0,
) -> None:
    if rot == 0.0:
        return _blt(x, y, img, u, v, w, h, colkey=colkey)
    nx, ny = x + w // 2, y + h // 2
    return blt_centered(nx, ny, img, u, v, w, h, colkey, rot)
