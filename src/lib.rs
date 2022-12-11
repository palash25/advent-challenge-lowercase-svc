use std::str;

use serde::{Serialize, Deserialize};
use anyhow::Result;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Deserialize)]
struct RequestBody {
    str: String
}

#[derive(Serialize)]
struct Resp {
    result: String
}

/// A simple Spin HTTP component.
#[http_component]
fn advent_challenge_lowercase_svc(req: Request) -> Result<Response> {
    if req.method() != http::Method::POST {    
        return bad_request()
    }

    let input: &str = "";
    match req.body() {
        Some(s) => {
            println!("{:?}", s);
            let input = match str::from_utf8(s) {
                Ok(s) => s,
                Err(e) => panic!("{}", e),
            };
            println!("{:?}", input);
        },
        None => {
            return bad_request()
        },
    };

    println!("{:?}", input);


    Ok(http::Response::builder()
        .status(200)
        .header("foo", "bar")
        .body(Some("Hello, Fermyon".into()))?)
}

fn bad_request() -> Result<Response> {
    Ok(http::Response::builder()
        .status(400)
        .body(Some("Bad Request".into()))?)
}