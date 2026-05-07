use std::time::Duration;

use serde::{Deserialize, Serialize};
use ureq::{RequestBuilder, typestate::WithoutBody};
use url::Url;

use crate::misc::config::config;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpConfig {
    pub user_agent: Option<String>,
    pub connect_timeout: Option<Duration>,
    pub read_timeout: Option<Duration>,
    pub request_timeout: Option<Duration>,
    pub max_redirects: Option<u32>,
    pub https_only: bool,
}

pub fn head(url: &Url) -> RequestBuilder<WithoutBody> {
    let config = config().http_config();
    let mut cfg = ureq::head(url.as_str())
        .config()
        .https_only(config.https_only);
    if let Some(timeout) = config.connect_timeout {
        cfg = cfg.timeout_connect(Some(timeout));
    }
    if let Some(timeout) = config.read_timeout {
        cfg = cfg.timeout_recv_response(Some(timeout));
    }
    if let Some(timeout) = config.request_timeout {
        cfg = cfg.timeout_global(Some(timeout));
    }
    if let Some(max_redirects) = config.max_redirects {
        cfg = cfg.max_redirects(max_redirects);
    }
    let mut req = cfg.build();
    if let Some(user_agent) = &config.user_agent {
        req = req.header("User-Agent", user_agent);
    }
    req
}

pub fn get(url: &Url) -> RequestBuilder<WithoutBody> {
    let config = config().http_config();
    let mut cfg = ureq::get(url.as_str())
        .config()
        .https_only(config.https_only);
    if let Some(timeout) = config.connect_timeout {
        cfg = cfg.timeout_connect(Some(timeout));
    }
    if let Some(timeout) = config.read_timeout {
        cfg = cfg.timeout_recv_response(Some(timeout));
    }
    if let Some(timeout) = config.request_timeout {
        cfg = cfg.timeout_global(Some(timeout));
    }
    if let Some(max_redirects) = config.max_redirects {
        cfg = cfg.max_redirects(max_redirects);
    }
    let mut req = cfg.build();
    if let Some(user_agent) = &config.user_agent {
        req = req.header("User-Agent", user_agent);
    }
    req
}
