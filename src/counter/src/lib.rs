mod http;

use handlebars::Handlebars;
use http::{HttpRequest, HttpResponse};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde_bytes::ByteBuf;
use serde_json::json;
use std::cell::RefCell;

const IMAGE_TEMPLATE: &str = include_str!("template.svg");

type Memory = VirtualMemory<DefaultMemoryImpl>;

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static COUNTER: RefCell<StableBTreeMap<String, u64, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),
        )
    );
}

fn find_header(headers: &[(String, String)], key: &str) -> Option<String> {
    headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == key.to_lowercase())
        .map(|(_, v)| v.to_string())
}

fn increase_counter(url: &str) -> u64 {
    COUNTER.with(|c| {
        let mut counter = c.borrow_mut();
        let count = counter.get(&String::from(url)).unwrap_or(0) + 1;
        counter.insert(url.to_string(), count);
        count
    })
}

#[ic_cdk::query]
pub fn http_request(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        status_code: 200_u16,
        headers: vec![],
        body: ByteBuf::new(),
        upgrade: Some(true),
    }
}

#[ic_cdk::update]
pub fn http_request_update(req: HttpRequest) -> HttpResponse {
    let referer: String =
        find_header(&req.headers, "referer").expect("A referer header is required");

    let count = increase_counter(&referer);

    let handlebars = Handlebars::new();

    let svg = handlebars
        .render_template(IMAGE_TEMPLATE, &json!({"counter": count.to_string()}))
        .unwrap();

    HttpResponse {
        status_code: 200_u16,
        headers: vec![
            (String::from("Content-Type"), String::from("image/svg+xml")),
            (
                String::from("Access-Control-Allow-Origin"),
                String::from("*"),
            ),
        ],
        body: ByteBuf::from(svg.as_bytes().to_vec()),
        upgrade: None,
    }
}
