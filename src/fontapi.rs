use std::{sync::Arc};

use fontdue::{
    layout::{GlyphPosition, Layout, TextStyle},
    Font,
};
use mini_moka::sync::Cache;
use numpy::ndarray::{Array2, ArrayViewMut2};
use palette::{
    blend::{BlendWith, PreAlpha},
    rgb::{Rgb},
    WithAlpha,
};
use pyo3::prelude::*;

pub struct CachedFont {
    pub(crate) font: fontdue::Font,
    pub(crate) cache: Cache<(char, u32), Arc<Array2<u8>>>,
}

impl CachedFont {
    pub fn new(font: fontdue::Font, max_size: u64) -> Self {
        let cache: Cache<(char, u32), Arc<Array2<u8>>> = Cache::builder()
            .max_capacity(max_size)
            .weigher(|_, v: &Arc<Array2<u8>>| -> u32 { v.len() as u32 })
            .build();
        Self { font, cache: cache }
    }

    pub fn try_from_bytes(
        bytes: &[u8],
        settings: fontdue::FontSettings,
        max_size: u64,
    ) -> anyhow::Result<Self> {
        let font = Font::from_bytes(bytes, settings).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Self::new(font, max_size))
    }

    pub fn rasterize(&mut self, ch: char, size: u32) -> Arc<Array2<u8>> {
        match self.cache.get(&(ch, size)) {
            Some(entry) => entry,
            None => {
                let (metrics, bitmap) = self.font.rasterize(ch, size as f32);
                let mut array =
                    Array2::<u8>::zeros((metrics.height as usize, metrics.width as usize));
                for y in 0..metrics.height as usize {
                    for x in 0..metrics.width as usize {
                        array[[y, x]] = bitmap[y * metrics.width as usize + x];
                    }
                }
                let a = Arc::new(array);
                self.cache.insert((ch, size), a.clone());
                a
            }
        }
    }

    pub fn rasterize_without_cache(&self, ch: char, size: u32) -> Array2<u8> {
        let (metrics, bitmap) = self.font.rasterize(ch, size as f32);
        let mut array = Array2::<u8>::zeros((metrics.height as usize, metrics.width as usize));
        for y in 0..metrics.height as usize {
            for x in 0..metrics.width as usize {
                array[[y, x]] = bitmap[y * metrics.width as usize + x];
            }
        }
        array
    }

    pub fn rasterize_text(&mut self, text: &str, size: u32) -> Array2<u8> {
        // do layout
        let mut layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        layout.append(
            std::slice::from_ref(&self.font),
            &TextStyle::new(text, size as f32, 0),
        );
        let glyph_positions: &Vec<GlyphPosition<()>> = layout.glyphs();
        let mut width = 0;
        let mut height = 0;
        for glyph in glyph_positions {
            width = width.max(glyph.x.max(0.0) as u32 + glyph.width as u32);
            height = height.max(glyph.y.max(0.0) as u32 + glyph.height as u32);
        }
        let mut array = Array2::<u8>::zeros((height as usize, width as usize));
        for glyph in glyph_positions {
            let bitmap = self.rasterize(glyph.parent, size);
            let x = glyph.x.max(0.0) as usize;
            let y = glyph.y.max(0.0) as usize;
            let width = glyph.width as usize;
            let height = glyph.height as usize;
            let mut x_offset = 0;
            let mut y_offset = 0;
            for y in y..y + height {
                for x in x..x + width {
                    array[[y, x]] = bitmap[[y_offset, x_offset]];
                    x_offset += 1;
                }
                x_offset = 0;
                y_offset += 1;
            }
        }
        array
    }
}

#[derive(Clone)]
pub struct Palette {
    pub palette: Vec<Rgb>,
}

fn color_dist(rgb1: (u8, u8, u8), rgb2: (u8, u8, u8)) -> f64 {
    let (r1, g1, b1) = rgb1;
    let (r2, g2, b2) = rgb2;
    let dx = (r1 as f64 - r2 as f64) * 0.30;
    let dy = (g1 as f64 - g2 as f64) * 0.59;
    let dz = (b1 as f64 - b2 as f64) * 0.11;
    dx * dx + dy * dy + dz * dz
}

impl Palette {
    pub fn closest_color(&self, color: Rgb) -> u8 {
        let mut min_dist = f64::INFINITY;
        let mut min_index = 0;
        for (i, &palette_color) in self.palette.iter().enumerate() {
            let dist = color_dist(
                (
                    (color.red * 255.0) as u8,
                    (color.green * 255.0) as u8,
                    (color.blue * 255.0) as u8,
                ),
                (
                    (palette_color.red * 255.0) as u8,
                    (palette_color.green * 255.0) as u8,
                    (palette_color.blue * 255.0) as u8,
                ),
            );
            if dist < min_dist {
                min_dist = dist;
                min_index = i;
            }
        }
        min_index as u8
    }
}

fn blend_mode(src: PreAlpha<Rgb>, dst: PreAlpha<Rgb>) -> PreAlpha<Rgb> {
    PreAlpha {
        color: Rgb::new(
            src.red + dst.red * (1.0 - src.alpha),
            src.green + dst.green * (1.0 - src.alpha),
            src.blue + dst.blue * (1.0 - src.alpha),
        ),
        alpha: dst.alpha,
    }
}

pub fn imprint_text(
    writer: &Palette,
    rasterized: Array2<u8>,
    text_color: u8,
    u: u32,
    v: u32,
    mut target: ArrayViewMut2<'_, u8>,
) {
    let rasterized_alpha = rasterized.mapv(|x| x as f32 / 255.0);
    for y in 0..rasterized.shape()[0] {
        for x in 0..rasterized.shape()[1] {
            let (ty, tx) = (v + y as u32, u + x as u32);
            if ty >= target.shape()[0] as u32 || tx >= target.shape()[1] as u32 {
                continue;
            }
            let intensity = rasterized_alpha[[y, x]];
            let c0 = writer.palette[text_color as usize]
                .with_alpha(intensity)
                .premultiply();
            let c1 = writer.palette[target[[ty as usize, tx as usize]] as usize]
                .with_alpha(1.0)
                .premultiply();
            let c2 = c0.blend_with(c1, blend_mode);
            target[[ty as usize, tx as usize]] = writer.closest_color(c2.color);
        }
    }
}

// #[pyfunction]
// pub fn fill_rect(x: u32, y: u32, width: u32, height: u32) -> PyResult<()> {
//     // let roboto_regular = Font::from_bytes(&[], fontdue::FontSettings::default()).unwrap();
//     let fonts = vec![roboto_regular];
//     let mut layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYUp);
//     layout.append(&fonts, &TextStyle::new("Hello ", 24.0, 0));
//     let glyphs = layout.glyphs();
//     for glyph in glyphs {}
//     Ok(())
// }
