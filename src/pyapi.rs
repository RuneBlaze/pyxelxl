use anyhow::Context;
use fontdue::FontSettings;
use numpy::{
    ndarray::{Array2, Dim},
    IntoPyArray, PyArray, PyArray2, PyArrayMethods, PyReadonlyArray2,
};
use palette::{encoding::linear, rgb::Rgb};
use parking_lot::Mutex;
use pyo3::prelude::*;
use rotsprite::rotsprite;
use std::sync::Arc;
use strum_macros::{Display, EnumString};

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
        layout: Option<&LayoutOpts>,
    ) -> Bound<'a, PyArray<u8, Dim<[usize; 2]>>> {
        let rarr = py.allow_threads(|| {
            self.inner
                .lock()
                .rasterize_text(text, size, layout.map(|l| l.to_layout_settings()))
        });
        rarr.into_pyarray_bound(py)
    }

    #[getter]
    pub fn name(&self) -> Option<String> {
        self.inner.lock().font.name().map(|s| s.to_string())
    }

    #[getter]
    pub fn capacity(&self) -> PyResult<u64> {
        Ok(self
            .inner
            .lock()
            .cache
            .policy()
            .max_capacity()
            .context("Cache is not bounded, not expected")?)
    }

    pub fn estimate_cached_bytes(&self) -> u64 {
        self.inner.lock().estimate_cached_bytes()
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

#[pyclass]
#[derive(EnumString, Display, Clone, PartialEq, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum HorzAlign {
    Left,
    Center,
    Right,
}

#[pyclass]
#[derive(EnumString, Display, Clone, PartialEq, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum VertAlign {
    Top,
    Center,
    Bottom,
}

#[pyclass]
pub struct LayoutOpts {
    pub max_width: Option<u32>,
    pub max_height: Option<u32>,
    pub horizontal_align: HorzAlign,
    pub vertical_align: VertAlign,
    pub line_height_mult: Option<f32>,
    pub can_break_words: Option<bool>,
}

#[pymethods]
impl LayoutOpts {
    #[new]
    pub fn new(
        max_width: Option<u32>,
        max_height: Option<u32>,
        horizontal_align: Option<&str>,
        vertical_align: Option<&str>,
        line_height_mult: Option<f32>,
        can_break_words: Option<bool>,
    ) -> PyResult<Self> {
        let h = horizontal_align
            .map(|s| s.parse())
            .transpose()
            .context("Invalid horizontal align")?
            .unwrap_or(HorzAlign::Left);
        let v = vertical_align
            .map(|s| s.parse())
            .transpose()
            .context("Invalid vertical align")?
            .unwrap_or(VertAlign::Top);
        if h != HorzAlign::Left && max_width.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "max_width must be specified when horizontal_align is not 'left'",
            ));
        }
        if v != VertAlign::Top && max_height.is_none() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "max_height must be specified when vertical_align is not 'top'",
            ));
        }
        Ok(Self {
            max_width,
            max_height,
            horizontal_align: h,
            vertical_align: v,
            line_height_mult,
            can_break_words,
        })
    }

    fn __repr__(&self) -> String {
        format!(
            "LayoutOpts(max_width={}, max_height={}, horizontal_align={}, vertical_align={}, line_height_mult={}, can_break_words={})",
            self.max_width.as_ref().map(|v| v.to_string()).unwrap_or("None".to_string()),
            self.max_height.as_ref().map(|v| v.to_string()).unwrap_or("None".to_string()),
            self.horizontal_align.to_string(),
            self.vertical_align.to_string(),
            self.line_height_mult.as_ref().map(|v| v.to_string()).unwrap_or("None".to_string()),
            self.can_break_words.as_ref().map(|v| v.to_string()).unwrap_or("None".to_string()),
        )
    }

    fn __str__(&self) -> String {
        self.__repr__()
    }
}

impl LayoutOpts {
    pub fn to_layout_settings(&self) -> fontdue::layout::LayoutSettings {
        let mut settings = fontdue::layout::LayoutSettings::default();
        settings.max_width = settings.max_width.or(self.max_width.map(|v| v as f32));
        settings.max_height = settings.max_height.or(self.max_height.map(|v| v as f32));
        settings.horizontal_align = match self.horizontal_align {
            HorzAlign::Left => fontdue::layout::HorizontalAlign::Left,
            HorzAlign::Center => fontdue::layout::HorizontalAlign::Center,
            HorzAlign::Right => fontdue::layout::HorizontalAlign::Right,
        };
        settings.vertical_align = match self.vertical_align {
            VertAlign::Top => fontdue::layout::VerticalAlign::Top,
            VertAlign::Center => fontdue::layout::VerticalAlign::Middle,
            VertAlign::Bottom => fontdue::layout::VerticalAlign::Bottom,
        };
        if let Some(line_height_mult) = self.line_height_mult {
            settings.line_height = line_height_mult;
        }
        if let Some(true) = self.can_break_words {
            settings.wrap_style = fontdue::layout::WrapStyle::Letter
        }
        settings
    }
}

#[pyfunction]
pub fn rotate<'a>(
    py: Python<'a>,
    buffer: PyReadonlyArray2<u8>,
    transparent: u8,
    rotation: f64,
) -> Bound<'a, PyArray<u8, Dim<[usize; 2]>>> {
    let width = buffer.as_array().shape()[1];
    let linear_buffer = buffer.as_array().to_slice().unwrap();
    let output = py.allow_threads(|| {
        let (width, height, output_buf) = rotsprite(linear_buffer, &transparent, width, rotation)
            .expect("Failed to rotate sprite");
        Array2::from_shape_vec((height, width), output_buf).unwrap()
    });
    output.into_pyarray_bound(py)
}
