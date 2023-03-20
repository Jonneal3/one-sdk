use std::io::Read;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::{IoStream, MessageExchange, EXCHANGE_MESSAGE};
use crate::{abi::Handle, encode_query, HeadersMultiMap, MultiMap};

define_exchange_core_to_host! {
    struct HttpCallRequest<'a> {
        kind: "http-call",
        /// HTTP method - will be used as-is.
        method: &'a str,
        url: &'a str,
        /// Headers.
        ///
        /// Multiple values for one key will not be joined.
        headers: &'a HeadersMultiMap,
        /// Body bytes to be sent.
        body: Option<&'a [u8]>
    } -> enum HttpCallResponse {
        Ok {
            #[serde(default)]
            request_body_stream: Option<IoStream>,
            handle: Handle,
        },
        Err {
            error: String
        }
    }
}
define_exchange_core_to_host! {
    struct HttpCallHeadRequest {
        kind: "http-call-head",
        /// Handle previously returned by `http-call`.
        handle: Handle
    } -> enum HttpCallHeadResponse {
        Ok {
            status: u16,
            headers: HeadersMultiMap,
            body_stream: IoStream, // TODO: optional? in case response doesn't have a body
        },
        Err {
            error: String
        }
    }
}

#[derive(Debug, Error)]
pub enum HttpCallError {
    #[error("HttpCall error: {0}")]
    Request(String), // TODO: more granular
    #[error("OutHttpCallHead error: {0}")]
    Response(String), // TODO: more granular
}
pub struct HttpRequest {
    handle: Handle,
}
impl HttpRequest {
    // TODO: proper errors
    pub fn fetch(
        method: &str,
        url: &str,
        headers: &HeadersMultiMap,
        query: &MultiMap,
        body: Option<&[u8]>,
    ) -> Result<Self, HttpCallError> {
        let url = match encode_query(query) {
            empty if empty.len() == 0 => url.to_string(),
            query => format!("{}?{}", url, query),
        };

        let response = HttpCallRequest {
            kind: HttpCallRequest::KIND,
            url: &url,
            method,
            headers,
            body,
        }
        .send_json(&EXCHANGE_MESSAGE)
        .unwrap();

        match response {
            HttpCallResponse::Ok {
                request_body_stream,
                handle,
            } => {
                assert!(request_body_stream.is_none());
                Ok(Self { handle })
            }
            HttpCallResponse::Err { error } => return Err(HttpCallError::Request(error)),
        }
    }

    // TODO: proper errors
    pub fn into_response(&mut self) -> Result<HttpResponse, HttpCallError> {
        let exchange_response = HttpCallHeadRequest::new(self.handle)
            .send_json(&EXCHANGE_MESSAGE)
            .unwrap();

        match exchange_response {
            HttpCallHeadResponse::Err { error } => return Err(HttpCallError::Response(error)),
            HttpCallHeadResponse::Ok {
                status,
                headers,
                body_stream,
            } => Ok(HttpResponse {
                status,
                headers,
                body: body_stream,
            }),
        }
    }
}

pub struct HttpResponse {
    status: u16,
    headers: HeadersMultiMap,
    body: IoStream,
}
impl HttpResponse {
    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn headers(&self) -> &HeadersMultiMap {
        &self.headers
    }

    #[allow(dead_code)]
    pub fn body(&mut self) -> impl Read + '_ {
        &mut self.body
    }

    // like <https://docs.rs/hyper/latest/hyper/struct.Response.html#method.into_body>
    pub fn into_body(self) -> IoStream {
        let HttpResponse { body, .. } = self;

        body
    }
}
