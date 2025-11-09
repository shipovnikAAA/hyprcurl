//! Python bindings using PyO3

use crate::curl::Curl as RustCurl;
use crate::error::CurlError;
use crate::types::HttpVersion;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Convert CurlError to PyErr
impl From<CurlError> for PyErr {
    fn from(err: CurlError) -> PyErr {
        PyException::new_err(err.to_string())
    }
}

/// Python wrapper for Curl
#[pyclass]
pub struct Curl {
    inner: RustCurl,
}

#[pymethods]
impl Curl {
    /// Create a new Curl instance
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Curl {
            inner: RustCurl::new()?,
        })
    }

    /// Set URL
    fn set_url(&mut self, url: &str) -> PyResult<()> {
        self.inner.set_url(url)?;
        Ok(())
    }

    /// Add header
    fn add_header(&mut self, header: &str) -> PyResult<()> {
        self.inner.add_header(header)?;
        Ok(())
    }

    /// Set timeout
    fn set_timeout(&mut self, timeout: i64) -> PyResult<()> {
        self.inner.setopt_long(crate::types::CurlOpt::Timeout, timeout)?;
        Ok(())
    }

    /// Set user agent
    fn set_user_agent(&mut self, ua: &str) -> PyResult<()> {
        self.inner.setopt_str(crate::types::CurlOpt::UserAgent, ua)?;
        Ok(())
    }

    /// Impersonate browser
    fn impersonate(&mut self, target: &str, default_headers: bool) -> PyResult<()> {
        self.inner.impersonate(target, default_headers)?;
        Ok(())
    }

    /// Perform request
    fn perform<'py>(&mut self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
        let mut buffer = Vec::new();

        // Release GIL during blocking I/O
        py.allow_threads(|| self.inner.perform(&mut buffer))?;

        Ok(PyBytes::new_bound(py, &buffer))
    }

    /// Get response code
    fn response_code(&self) -> PyResult<i64> {
        Ok(self.inner.response_code()?)
    }

    /// Get total time
    fn total_time(&self) -> PyResult<f64> {
        Ok(self.inner.total_time()?)
    }

    /// Get effective URL
    fn effective_url(&self) -> PyResult<String> {
        Ok(self.inner.effective_url()?)
    }

    /// Reset handle
    fn reset(&mut self) {
        self.inner.reset();
    }

    /// Get version
    #[staticmethod]
    fn version() -> String {
        crate::version()
    }
}

/// Quick request function (convenience)
#[pyfunction]
fn get(url: &str) -> PyResult<Vec<u8>> {
    let mut curl = RustCurl::new()?;
    curl.set_url(url)?;
    let mut buffer = Vec::new();
    curl.perform(&mut buffer)?;
    Ok(buffer)
}

/// Initialize the Python module
#[pymodule]
fn curl_cffi_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Curl>()?;
    m.add_function(wrap_pyfunction!(get, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_curl_new() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|_py| {
            let curl = Curl::new();
            assert!(curl.is_ok());
        });
    }
}
