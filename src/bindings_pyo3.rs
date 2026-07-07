use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
#[derive(Clone)]
pub struct PreprocessConfigPy {
    #[pyo3(get, set)]
    pub window_size: Option<u32>,
    #[pyo3(get, set)]
    pub k: Option<f64>,
    #[pyo3(get, set)]
    pub target_width: Option<u32>,
}

#[pymethods]
impl PreprocessConfigPy {
    #[new]
    #[pyo3(signature = (window_size=None, k=None, target_width=None))]
    fn new(window_size: Option<u32>, k: Option<f64>, target_width: Option<u32>) -> Self {
        PreprocessConfigPy {
            window_size,
            k,
            target_width,
        }
    }
}

#[pyfunction]
#[pyo3(name = "preprocess_image", signature = (data, config=None))]
fn preprocess_image_py(py: Python<'_>, data: Vec<u8>, config: Option<PreprocessConfigPy>) -> PyResult<PyObject> {
    let (window_size, k_param, user_target_width) = match config {
        Some(cfg) => (
            cfg.window_size.unwrap_or(25),
            cfg.k.unwrap_or(0.2) as f32,
            cfg.target_width,
        ),
        None => (25, 0.2f32, None),
    };

    match crate::preprocess_image_sync(data, window_size, k_param, user_target_width) {
        Ok(buf) => {
            let bytes = PyBytes::new_bound(py, &buf);
            Ok(bytes.into())
        }
        Err(e) => Err(pyo3::exceptions::PyValueError::new_err(e)),
    }
}

#[pymodule]
fn bgustreadimg(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PreprocessConfigPy>()?;
    m.add_function(wrap_pyfunction!(preprocess_image_py, m)?)?;
    Ok(())
}
