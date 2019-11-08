extern crate wee_alloc;
#[macro_use]
extern crate http_guest;
use http_guest::{Request, RequestExt, Response};
use scraper::{Html, Selector};
use serde_derive::Serialize;
use serde_json;
use std::collections::HashMap;
use url::Url;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize)]
struct ElementResponse {
    inner_html: String,
    inner_text: Vec<String>,
}
#[derive(Debug, Serialize)]
struct ResponseBody {
    selector: String,
    elements: Vec<ElementResponse>,
}

fn extract_selector(html: &String, selector_str: &str) -> ResponseBody {
    let fragment = Html::parse_document(html);
    let selector = Selector::parse(selector_str).unwrap();
    let elements: Vec<_> = fragment
        .select(&selector)
        .map(|el| ElementResponse {
            inner_html: el.inner_html(),
            inner_text: el.text().map(|text| text.to_string()).collect(),
        })
        .collect();
    let res = ResponseBody {
        selector: selector_str.to_string(),
        elements,
    };
    res
}

fn handler(request: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    // TODO: handle `#id` selectors
    let uri_str = request.uri().to_string();
    let uri_str = if uri_str.chars().next().map(|c| c == '/') == Some(true) {
        format!("http://example.com/{}", uri_str)
    } else {
        uri_str
    };
    let url = Url::parse(&uri_str).expect(&format!("Parse url failed - {}", &uri_str));
    let hash_query: HashMap<_, _> = url.query_pairs().to_owned().collect();
    match (hash_query.get("url"), hash_query.get("selector")) {
        (Some(ref url), Some(ref selector)) => {
            let url = format!("http://{}", url);

            let res = Request::builder()
                .method("GET")
                .header("accept", "text/html")
                .uri(url.as_str())
                .body(vec![])
                .expect("setting url works")
                .send()
                .expect("Failed to send HTTP request");

            assert_eq!(res.status(), 200);
            let page_html = String::from_utf8_lossy(res.body()).into_owned();
            let content = extract_selector(&page_html, selector);
            let content_str =
                serde_json::to_string_pretty(&content).expect("Failed to serialize to JSON");
            Response::builder()
                .status(200)
                .header("Content-type", "application/json")
                .body(content_str.into_bytes())
                .expect("failed to render response")
        }
        _ => Response::builder()
            .status(403)
            .body(b"`selector` and `url` are required query params".to_vec())
            .unwrap(),
    }
}

guest_app!(handler);

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
