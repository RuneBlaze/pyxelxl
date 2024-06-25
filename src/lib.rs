pub mod fontapi;
pub mod pyapi;
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn pyxelxl(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<pyapi::Font>()?;
    m.add_class::<pyapi::FontDrawer>()?;
    Ok(())
}
