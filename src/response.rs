use http::HeaderMap;
use url::Url;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ResponseUrl(pub Url);

/// Extension trait for http::response::Builder objects
///
/// Allows the user to add a `Url` to the http::Response
pub trait ResponseBuilderExt {
    /// A builder method for the `http::response::Builder` type that allows the user to add a `Url`
    /// to the `http::Response`
    fn url(self, url: Url) -> Self;

    /// Add a set of Headers to the existing ones on this `http::Response`.
    ///
    /// The headers will be merged in to any already set.
    fn headers(self, headers: HeaderMap) -> Self;
}

impl ResponseBuilderExt for http::response::Builder {
    fn url(self, url: Url) -> Self {
        self.extension(ResponseUrl(url))
    }

    fn headers(mut self, headers: HeaderMap) -> Self {
        if let Some(target) = self.headers_mut() {
            crate::util::replace_headers(target, headers);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use std::iter::FromIterator;

    use super::{ResponseBuilderExt, ResponseUrl};
    use http::{response::Builder, HeaderMap, HeaderValue};
    use url::Url;

    #[test]
    fn test_response_builder_ext() {
        let url = Url::parse("http://example.com").unwrap();
        let response = Builder::new()
            .status(200)
            .url(url.clone())
            .body(())
            .unwrap();

        assert_eq!(
            response.extensions().get::<ResponseUrl>(),
            Some(&ResponseUrl(url))
        );
    }

    #[test]
    fn test_response_builder_ext_headers() {
        let response = Builder::new()
            .status(200)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .header(hyper::header::CONTENT_LENGTH, 42)
            .headers(HeaderMap::from_iter([
                (hyper::header::CONTENT_TYPE, HeaderValue::from_static("xyz")),
                (hyper::header::ETAG, HeaderValue::from_static("abcd")),
            ]))
            .body(())
            .unwrap();

        let expected = [
            (hyper::header::CONTENT_TYPE, HeaderValue::from_static("xyz")),
            (
                hyper::header::CONTENT_LENGTH,
                HeaderValue::from_static("42"),
            ),
            (hyper::header::ETAG, HeaderValue::from_static("abcd")),
        ];

        assert_eq!(response.headers(), &HeaderMap::from_iter(expected));
    }
}
