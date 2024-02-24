use candid::{CandidType, Deserialize};
use serde_bytes::ByteBuf;
use serde_querystring::UrlEncodedQS;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
    pub certificate_version: Option<u16>,
}

pub type HeaderField = (String, String);

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
    pub upgrade: Option<bool>,
}

pub fn find_header(headers: &[(String, String)], key: &str) -> Option<String> {
    headers
        .iter()
        .find(|(k, _)| k.to_lowercase() == key.to_lowercase())
        .map(|(_, v)| v.to_string())
}

pub fn qs_param_exists(url: &str, key: &str) -> bool {
    url.split('?').last().map_or(false, |qs| {
        let parsed_qs = UrlEncodedQS::parse(qs.as_bytes());
        parsed_qs
            .keys()
            .iter()
            .any(|k| k.eq_ignore_ascii_case(key.as_bytes()))
    })
}
