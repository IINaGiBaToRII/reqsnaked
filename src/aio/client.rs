use pyo3::prelude::*;

use crate::aio::response::AsyncResponse;
use crate::aio::request::AsyncRequest;


#[pyclass]
pub struct AsyncClient {
    client: reqwest::Client,
}


impl AsyncClient {
    pub fn py_awaitable_request<'rt>(
        &self,
        client: reqwest::Client,
        request: reqwest::Request,
        py: Python<'rt>
    )
    -> PyResult<&'rt PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let response = client.execute(request).await.unwrap();
            Ok(AsyncResponse {
                status: response.status(),
                response: std::cell::RefCell::new(Some(response)),
            })
        })
    }
}

#[pymethods]
impl AsyncClient {
    #[new]
    pub fn new(
        user_agent: Option<String>,
        headers: Option<std::collections::HashMap<String, String>>
    ) -> Self {
        let mut client = reqwest::Client::builder();
        if let Some(ref user_agent) = user_agent {
            client = client.user_agent(user_agent);
        }
        let mut default_headers_map = reqwest::header::HeaderMap::new();
        if let Some(default_headers) = headers {
            for (key, value) in default_headers {
            // TODO: headers wrapper
                default_headers_map.insert(
                    reqwest::header::HeaderName::from_bytes(key.as_bytes()).unwrap(),
                    reqwest::header::HeaderValue::from_str(&value).unwrap()
                );
            }
            client = client.default_headers(default_headers_map);
        }

        AsyncClient {
            client: client.build().unwrap(),
        }
    }

    pub fn request<'rt>(
        &self,
        request: &PyCell<AsyncRequest>,
        py: Python<'rt>
    ) -> PyResult<&'rt PyAny> {
        let client = self.client.clone();
        let request = request.borrow().build(&client);
        self.py_awaitable_request(client, request, py)
    }
}


pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "client")?;
    submod.add_class::<AsyncClient>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
