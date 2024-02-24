use crate::http::HttpResponse;
use crate::{COUNTER, IMAGE_TEMPLATE};
use handlebars::Handlebars;
use serde_bytes::ByteBuf;
use serde_json::json;

pub struct Counter;

impl Counter {
    pub fn get(url: &str) -> u64 {
        COUNTER.with(|c| c.borrow().get(&String::from(url)).unwrap_or(0))
    }

    pub fn increase(url: &str) -> u64 {
        COUNTER.with(|c| {
            let mut counter = c.borrow_mut();
            let count = counter.get(&String::from(url)).unwrap_or(0) + 1;
            counter.insert(url.to_string(), count);
            count
        })
    }

    pub fn svg_http_response(count: u64) -> HttpResponse {
        let handlebars = Handlebars::new();

        let svg = handlebars
            .render_template(IMAGE_TEMPLATE, &json!({"counter": count.to_string()}))
            .unwrap();

        HttpResponse {
            status_code: 200_u16,
            headers: vec![(String::from("Content-Type"), String::from("image/svg+xml"))],
            body: ByteBuf::from(svg.as_bytes().to_vec()),
            upgrade: None,
        }
    }
}
