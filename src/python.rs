use piet::{FillRule, RenderContext};
use piet_common::Piet;
use kurbo::Rect;

use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[pyclass]
pub struct Canvas {
    ctx: &'static mut Piet<'static>,
}

impl Canvas {
   pub fn new<'a, 'b>(inner: &'b mut Piet<'a>) -> Self {
        let inner = unsafe { std::mem::transmute::<&'b mut Piet<'a>, &'static mut Piet<'static>>(inner) };
        Canvas { ctx: inner }
    }
}

#[pymethods]
impl Canvas {
    fn fill_rect(&mut self, rect: (f64, f64, f64, f64), color: (f64, f64, f64, f64)) -> PyResult<()> {
        let rect = Rect::from_origin_size((rect.0, rect.1).into(), (rect.2, rect.3).into());
        let color = rgba_float_to_u32(color);
        let brush = self.ctx.solid_brush(color).unwrap();
        self.ctx.fill(rect, &brush, FillRule::NonZero);
        Ok(())
    }

}

pub fn run_python<'a, 'b>(ctx: &'b mut Piet<'a>, code: &str) -> PyResult<()> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let canvas = Canvas::new(ctx);
    let obj = PyRef::new(py, canvas).unwrap();
    let locals = [("canvas", obj)].into_py_dict(py);
    py.run(code, None, Some(&locals))
}

fn rgba_float_to_u32(color: (f64, f64, f64, f64)) -> u32 {
    debug_assert!(color.0 >= 0.0 && color.0 <= 1.0);
    debug_assert!(color.1 >= 0.0 && color.0 <= 1.0);
    debug_assert!(color.2 >= 0.0 && color.0 <= 1.0);
    debug_assert!(color.3 >= 0.0 && color.0 <= 1.0);
    let mut out = (255. * color.3) as u32;
    out = out | ((255. * color.0) as u32) << 24;
    out = out | ((255. * color.1) as u32) << 16;
    out = out | ((255. * color.2) as u32) << 8;
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rbga_conv() {
        assert_eq!(rgba_float_to_u32((0.0, 0.0, 0.0, 1.0)), 0x00_00_00_ff);
        assert_eq!(rgba_float_to_u32((1.0, 0.0, 0.0, 0.0)), 0xff_00_00_00);
        assert_eq!(rgba_float_to_u32((0.0, 1.0, 0.0, 0.0)), 0x00_ff_00_00);
    }
}
