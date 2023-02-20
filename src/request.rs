use pyo3::prelude::*;

use crate::{
    primitives::{self, BasicAuth},
    py2rs::{base::ToNative, duration::PyDurationAnalog},
};

#[pyclass]
pub struct Request {
    pub method: primitives::HTTPMethod,
    pub url: primitives::URL,
    pub headers: Option<std::collections::HashMap<String, String>>,
    pub query: Option<std::collections::HashMap<String, String>>,
    pub bearer_auth: Option<String>,
    pub body: Option<Vec<u8>>,
    pub timeout: Option<std::time::Duration>,
    pub basic_auth: Option<BasicAuth>,
}

impl Request {
    pub fn build(&self, client: &reqwest::Client) -> PyResult<reqwest::Request> {
        let mut request = client.request(self.method.to_native()?, self.url.to_native()?);
        if let Some(ref headers) = self.headers {
            for (key, value) in headers.iter() {
                request = request.header(key, value);
            }
        }
        if let Some(ref query) = self.query {
            request = request.query(query);
        }
        if let Some(ref bearer_auth) = self.bearer_auth {
            request = request.bearer_auth(bearer_auth);
        }
        if let Some(ref body) = self.body {
            request = request.body(body.clone());
        }
        if let Some(ref timeout) = self.timeout {
            request = request.timeout(*timeout);
        }
        if let Some(ref basic_auth) = self.basic_auth {
            request = request.basic_auth(basic_auth.username.clone(), basic_auth.password.clone());
        }

        Ok(request.build().unwrap())
    }
}

#[pymethods]
impl Request {
    // TODO: More params
    #[new]
    pub fn new(
        method: primitives::HTTPMethod,
        url: primitives::URL,
        headers: Option<std::collections::HashMap<String, String>>,
        query: Option<std::collections::HashMap<String, String>>,
        bearer_auth: Option<String>,
        body: Option<Vec<u8>>,
        timeout: Option<PyDurationAnalog>,
        basic_auth: Option<BasicAuth>,
    ) -> PyResult<Self> {
        Ok(Request {
            method,
            url,
            headers,
            query,
            bearer_auth,
            body,
            basic_auth,
            timeout: {
                if let Some(inner) = timeout {
                    match inner.to_native() {
                        Ok(value) => Some(value),
                        Err(_) => None, // TODO
                    }
                } else {
                    None
                }
            },
        })
    }
}

pub fn init_module(py: Python, parent_module: &PyModule, library: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "request")?;
    submod.add_class::<Request>()?;
    library.add_class::<Request>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
