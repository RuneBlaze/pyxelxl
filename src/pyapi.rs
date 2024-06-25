use fontdue::FontSettings;
use numpy::{ndarray::Dim, IntoPyArray, PyArray, PyArray2, PyArrayMethods, PyReadonlyArray2};
use palette::rgb::Rgb;
use parking_lot::Mutex;
use pyo3::prelude::*;
use std::sync::Arc;

use crate::fontapi::{imprint_text, CachedFont, Palette};

#[pyclass]
pub struct Font {
    pub(crate) inner: Arc<Mutex<CachedFont>>,
}

#[pymethods]
impl Font {
    #[new]
    pub fn new(bytes: &[u8], capacity: Option<u64>) -> PyResult<Self> {
        let font = CachedFont::try_from_bytes(
            bytes,
            FontSettings::default(),
            capacity.unwrap_or(32 * 1024 * 1024),
        )
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e.to_string()))?;
        Ok(Self {
            inner: Arc::new(Mutex::new(font)),
        })
    }

    pub fn rasterize_text<'a>(
        &'a self,
        py: Python<'a>,
        text: &str,
        size: u32,
    ) -> Bound<'a, PyArray<u8, Dim<[usize; 2]>>> {
        self.inner
            .lock()
            .rasterize_text(text, size)
            .into_pyarray_bound(py)
    }
}

#[pyclass]
pub struct FontDrawer {
    palette: Palette,
}

#[pymethods]
impl FontDrawer {
    #[new]
    pub fn new(colors: Vec<u32>) -> Self {
        let colors = colors
            .into_iter()
            .map(|color| {
                let r = color >> 16 & 0xFF;
                let g = color >> 8 & 0xFF;
                let b = color & 0xFF;
                Rgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
            })
            .collect();
        Self {
            palette: Palette::from_colors(colors),
        }
    }

    pub fn set_allow(&mut self, allows: Vec<usize>) {
        let mut allow = vec![false; self.palette.colors.len()];
        for i in allows {
            allow[i as usize] = true;
        }
        self.palette.allow = allow;
    }

    pub fn reset_allow(&mut self) {
        self.palette.allow = vec![true; self.palette.colors.len()];
    }

    pub fn imprint(
        &self,
        rasterized: PyReadonlyArray2<u8>,
        text_color: u8,
        u: u32,
        v: u32,
        target: &Bound<'_, PyArray2<u8>>,
    ) {
        let rasterized = rasterized.as_array().to_owned();
        let target = unsafe { target.as_array_mut() };
        imprint_text(&self.palette, rasterized, text_color, u, v, target);
    }

    pub fn __len__(&self) -> usize {
        self.palette.colors.len()
    }
}
