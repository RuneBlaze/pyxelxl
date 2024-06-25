from __future__ import annotations

from collections import Counter
from functools import lru_cache, partial
from typing import List, Optional, Protocol, Union

import numpy as np
import pyxel

from pyxelxl.pyxelxl import Font as _Font
from pyxelxl.pyxelxl import FontDrawer, LayoutOpts


@lru_cache
def _state():
    return FontDrawer(pyxel.colors.to_list())


def _image_as_ndarray(image: pyxel.Image) -> np.ndarray:
    data_ptr = image.data_ptr()
    h, w = image.height, image.width
    return np.frombuffer(data_ptr, dtype=np.uint8).reshape(h, w)


def _five_point_consensus(arr: np.ndarray) -> Optional[int]:
    # Given two dimensional array, sample four corners and center of the image, and return the mode
    # of the five points.
    h, w = arr.shape
    if h < 2 or w < 2:
        return None
    counts = Counter(
        [
            arr[0, 0],
            arr[0, w // 2],
            arr[0, w - 1],
            arr[h - 1, 0],
            arr[h - 1, w - 1],
        ]
    )
    return counts.most_common(1)[0][0]


class DrawTextLike(Protocol):
    """
    A function like `pyxel.text`, but with layout options enabled.
    """

    def __call__(
        self,
        x: int,
        y: int,
        s: str,
        col: int,
        layout: Optional[LayoutOpts] = ...,
        /,
    ) -> None:
        ...


class Font:
    """
    TrueType font rendering for Pyxel. This class wraps a .ttf font file and provides
    methods for rendering text onto the Pyxel screen.

    Example:
    ```python
    import pyxel
    from pyxelxl import Font, LayoutOpts
    # After Pyxel is initialized
    # .. init
    font = Font("/path/to/font.ttf")
    font_text = font.specialize(font_size=12, threshold=100)
    # .. draw
    font_text(0, 0, "Hello, World!", 7)
    ```
    """

    def __init__(
        self, path_like: Union[str, bytes], max_cached_bytes: int = 32 * 1024 * 1024
    ):
        """
        Initializes a new Font object with a path to a TTF font or font data.

        :param path_like: Path to the font file or bytes object containing font data.
        :param max_cached_bytes: Maximum number of bytes used for caching font data.
        """
        if isinstance(path_like, str):
            with open(path_like, "rb") as fh:
                buf = fh.read()
        else:
            buf = path_like
        self.inner = _Font(buf, max_cached_bytes)

    def text(
        self,
        x: int,
        y: int,
        s: str,
        col: int,
        font_size: int,
        dithering_cols: Optional[List[int]] = None,
        threshold: Optional[int] = None,
        layout: Optional[LayoutOpts] = None,
    ):
        """
        Draws text onto the Pyxel screen at the specified location.

        Args:
            x (int): The x-coordinate of the text.
            y (int): The y-coordinate of the text.
            s (str): The text to draw.
            col (int): The color of the text.
            font_size (int): The size of the font.
            dithering_cols (Optional[List[int]]): A list of palette colors to use for dithering.
            threshold (Optional[int]): The threshold for dithering, ranges in 0-255.
            layout (Optional[LayoutOpts]): The layout options for the text.
        """
        rasterized = self._rasterize(s, font_size, layout=layout)
        drawer = _state()
        if dithering_cols is None:
            dithering_cols = [0, 7, 13]
        if col not in dithering_cols:
            dithering_cols.append(col)
        reserved = next(iter(set(range(len(drawer))) - set(dithering_cols)))
        temporary_buffer = pyxel.Image(*rasterized.shape[::-1])
        temp_buffer_arr = _image_as_ndarray(temporary_buffer)
        if not np.all((rasterized == 0) | (rasterized == 255)) and not threshold:
            temporary_buffer.blt(0, 0, pyxel.screen, x, y, *rasterized.shape[::-1])
        else:
            temp_buffer_arr[:] = reserved
        if threshold is None:
            if len(dithering_cols) < len(drawer) - 1:
                consensus = _five_point_consensus(temp_buffer_arr)
                if consensus is not None and consensus not in dithering_cols:
                    dithering_cols.append(consensus)
            if len(dithering_cols) >= len(drawer):
                raise ValueError(
                    "Too many dithering colors; need to reserve one for transparency."
                )
            drawer.set_allow(dithering_cols)
            drawer.imprint(rasterized, col, 0, 0, temp_buffer_arr)
            temp_buffer_arr[~np.isin(temp_buffer_arr, dithering_cols)] = reserved
        else:
            temp_buffer_arr[rasterized > threshold] = col
        pyxel.blt(
            x, y, temporary_buffer, 0, 0, *rasterized.shape[::-1], colkey=reserved
        )

    def _rasterize(
        self, text: str, font_size: int, layout: Optional[LayoutOpts] = None
    ) -> np.ndarray:
        return self.inner.rasterize_text(text, font_size, layout)

    def rasterize(
        self,
        text: str,
        font_size: int,
        threshold: int,
        fg_col: int,
        bg_col: int,
        layout: Optional[LayoutOpts] = None,
    ) -> pyxel.Image:
        rasterized = self._rasterize(text, font_size, layout)
        w, h = rasterized.shape[::-1]
        image = pyxel.Image(w, h)
        arr = _image_as_ndarray(image)
        arr[:] = np.where(rasterized > threshold, fg_col, bg_col)
        return image

    def specialize(self, font_size: int, threshold: int = None) -> DrawTextLike:
        """
        Creates a `pyxel.text`-like function by creating a partial function with the specified font size and threshold.

        Args:
            font_size (int): The size of the font.
            threshold (int, optional): The threshold value for the specialized font to discretize the font.
                If set, disables antialiasing and defaults to None.

        Returns:
            DrawTextLike: A partial function that can be used to draw text with the specialized font.
                This function has the same call signature as `pyxel.text`, but enables layout options.

        """
        return partial(self.text, font_size=font_size, threshold=threshold)

    def estimate_cached_bytes(self) -> Optional[int]:
        """
        Estimates the number of bytes used for caching font glyphs.

        Returns:
            int | None: The number of bytes used for caching font glyphs, or None if the estimate is not available.
        """
        return self.inner.estimate_cached_bytes()

    @property
    def name(self) -> Optional[str]:
        """
        Returns the name of the font, if available.

        Returns:
            Optional[str]: The name of the font.
        """
        return self.inner.name

    @property
    def max_cached_bytes(self) -> int:
        """
        Returns the maximum number of bytes used for caching font glyphs.

        Returns:
            int: The maximum number of bytes used for caching font glyphs.
        """
        return self.inner.capacity

    def __str__(self) -> str:
        return f"Font({self.name})"
