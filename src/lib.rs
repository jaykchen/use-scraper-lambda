use lambda_flows::{request_received, send_response};
use serde_json::Value;
use std::collections::HashMap;
use tokio;
use web_scraper_flows::get_page_text;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() {
    request_received(handler).await;
}

async fn handler(_qry: HashMap<String, Value>, _body: Vec<u8>) {
    let url = _qry.get("url").unwrap().as_str().unwrap();
    match get_page_text(&url).await {
        Ok(text) => send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            text.as_bytes().to_vec(),
        ),

        Err(_e) => send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            _e.as_bytes().to_vec(),
        ),
    };
}

// set_response_status(status as i32);
// set_response_headers(headers.as_ptr(), headers.len() as i32);
// set_response(body.as_ptr(), body.len() as i32);
