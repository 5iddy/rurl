use url::Url;
use reqwest::header::HeaderMap;
// use reqwest::

struct Request {
    url: Url,
    headers: HeaderMap
}