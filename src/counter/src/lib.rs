mod counter;
mod http;

use counter::Counter;
use http::{find_header, qs_param_exists, HttpRequest, HttpResponse};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{DefaultMemoryImpl, StableBTreeMap};
use serde_bytes::ByteBuf;
use std::cell::RefCell;

const IMAGE_TEMPLATE: &str = include_str!("counter.svg");

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

#[ic_cdk::query]
pub fn http_request(req: HttpRequest) -> HttpResponse {
    let url: String = find_header(&req.headers, "referer").expect("Referer header is missing.");

    if qs_param_exists(&req.url, "track") {
        return HttpResponse {
            status_code: 200_u16,
            headers: vec![],
            body: ByteBuf::new(),
            upgrade: Some(true),
        };
    }

    let count = Counter::get(&url);
    Counter::svg_http_response(count)
}

#[ic_cdk::update]
pub fn http_request_update(req: HttpRequest) -> HttpResponse {
    let url: String = find_header(&req.headers, "referer").expect("Referer header is missing.");

    let count = Counter::increase(&url);
    Counter::svg_http_response(count)
}
