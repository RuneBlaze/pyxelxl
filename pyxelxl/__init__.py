from .font import Font
from .pyxelxl import LayoutOpts
from .rotate import blt as blt_rot

from typing import Literal, Optional

def layout(
    w: Optional[int] = None,
    h: Optional[int] = None,
    ha: Literal["left", "center", "right"] = "left",
    va: Literal["top", "center", "bottom"] = "top",
    line_height: Optional[float] = None,
    break_words: Optional[bool] = None,
) -> LayoutOpts:
    """
    Create a layout with specified options.

    Args:
        w (Optional[int]): The maximum width of the layout. Defaults to None.
        h (Optional[int]): The maximum height of the layout. Defaults to None.
        ha (Literal["left", "center", "right"]): The horizontal alignment of the layout. Defaults to "left".
        va (Literal["top", "center", "bottom"]): The vertical alignment of the layout. Defaults to "top".
        line_height (Optional[float]): The line height multiplier of the layout. Defaults to None.
        break_words (Optional[bool]): Whether words can be broken in the layout.
            Defaults to None (not breaking words).

    Returns:
        LayoutOpts: The layout options object.

    """
    return LayoutOpts(
        max_width=w,
        max_height=h,
        horizontal_align=ha,
        vertical_align=va,
        line_height_mult=line_height,
        can_break_words=break_words,
    )

__all__ = ["Font", "LayoutOpts", "blt_rot", "layout"]
