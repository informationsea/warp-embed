//! # warp-embed
//!
//! Serve [embedded file](https://crates.io/crates/rust-embed) with [warp](https://crates.io/crates/warp)
//!
//! ```
//! use warp::Filter;
//! use rust_embed::RustEmbed;
//!
//! #[derive(RustEmbed)]
//! #[folder = "data"]
//! struct Data;
//!
//! let data_serve = warp_embed::embed(&Data);
//! ```

use std::borrow::Cow;
use std::sync::Arc;
use warp::filters::path::{FullPath, Tail};
use warp::http::Uri;
use warp::{reject::Rejection, reply::Reply, reply::Response, Filter};

/// Embed serving configuration
#[derive(Debug, Clone)]
pub struct EmbedConfig {
    /// list of directory index.
    ///
    /// Default value is `vec!["index.html".to_string(), "index.htm".to_string()]`
    pub directory_index: Vec<String>,
}

impl Default for EmbedConfig {
    fn default() -> Self {
        EmbedConfig {
            directory_index: vec!["index.html".to_string(), "index.htm".to_string()],
        }
    }
}

struct EmbedFile {
    data: Cow<'static, [u8]>,
}

impl Reply for EmbedFile {
    fn into_response(self) -> Response {
        Response::new(self.data.into())
    }
}

fn append_filename(path: &str, filename: &str) -> String {
    if path.is_empty() {
        filename.to_string()
    } else {
        format!("{}/{}", path, filename)
    }
}

/// Creates a `Filter` that always serves one embedded file
pub fn embed_one<A: rust_embed::RustEmbed>(
    _: &A,
    filename: &str,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let filename = Arc::new(filename.to_string());
    warp::any()
        .map(move || filename.clone())
        .and_then(|filename: Arc<String>| async move {
            if let Some(x) = A::get(&filename) {
                Ok(create_reply(x, &filename))
            } else {
                Err(warp::reject::not_found())
            }
        })
}

/// Creates a `Filter` that serves embedded files at the base `path` joined by the request path.
pub fn embed<A: rust_embed::RustEmbed>(
    x: &A,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    embed_with_config(x, EmbedConfig::default())
}

#[derive(Debug)]
struct NotFound {
    config: Arc<EmbedConfig>,
    tail: Tail,
    full: FullPath,
}

impl warp::reject::Reject for NotFound {}

/// Creates a `Filter` that serves embedded files at the base `path` joined
/// by the request path with configuration.
pub fn embed_with_config<A: rust_embed::RustEmbed>(
    _: &A,
    config: EmbedConfig,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let config = Arc::new(config);
    let config2 = config.clone();
    let direct_serve = warp::path::tail().and_then(|tail: Tail| async move {
        if let Some(x) = A::get(tail.as_str()) {
            Ok(create_reply(x, tail.as_str()))
        } else {
            Err(warp::reject::not_found())
        }
    });

    let directory_index = warp::any()
        .map(move || config.clone())
        .and(warp::path::tail())
        .and(warp::path::full())
        .and_then(
            |config: Arc<EmbedConfig>, tail: Tail, full: FullPath| async move {
                for one in config.directory_index.iter() {
                    if let Some(x) = A::get(&append_filename(tail.as_str(), one)) {
                        if full.as_str().ends_with('/') {
                            return Ok(create_reply(x, one));
                        }
                    }
                }

                Err(warp::reject::not_found())
            },
        );

    let redirect = warp::any()
        .map(move || config2.clone())
        .and(warp::path::tail())
        .and(warp::path::full())
        .and_then(
            |config: Arc<EmbedConfig>, tail: Tail, full: FullPath| async move {
                for one in config.directory_index.iter() {
                    if A::get(&append_filename(tail.as_str(), one)).is_some()
                        && !full.as_str().ends_with('/')
                    {
                        let new_path = format!("{}/", full.as_str());
                        return Ok(warp::redirect(
                            Uri::builder()
                                .path_and_query(new_path.as_str())
                                .build()
                                .unwrap(),
                        ));
                    }
                }

                Err(warp::reject::not_found())
            },
        );

    warp::any()
        .and(direct_serve)
        .or(directory_index)
        .or(redirect)
}

fn create_reply(data: Cow<'static, [u8]>, actual_name: &str) -> impl Reply {
    let suggest = mime_guess::guess_mime_type(actual_name);
    warp::reply::with_header(EmbedFile { data }, "Content-Type", suggest.to_string())
}

#[cfg(test)]
mod test;
