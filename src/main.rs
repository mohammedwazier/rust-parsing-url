use std::collections::HashMap;

use axum::{
    extract::{Query, RawQuery},
    routing::get,
    Router,
};

use tokio::main;

#[main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/query", get(get_query))
        .route("/testing", get(handler));

    println!("Running on localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap()
}

async fn hello_world() -> &'static str {
    "Hello World!"
}

async fn get_query(params: Query<HashMap<String, String>>) -> Result<(), ()> {
    dbg!(params);
    Ok(())
}

struct UserParam {
    user_id: Vec<String>,
}

fn parse_query(param: &str) -> HashMap<&str, Vec<&str>> {
    let mut url_value = HashMap::new();

    for pair in param.split('&') {
        let parts: Vec<&str> = pair.split('=').collect();
        if parts.len() == 2 {
            let key = parts[0];
            let value = parts[1];
            url_value
                .entry(key)
                .and_modify(|v: &mut Vec<&str>| v.push(value))
                .or_insert_with(|| vec![value]);
        }
    }
    url_value
}

async fn handler(param: RawQuery) -> String {
    // localhost:3000/testing?user_id=bbbbb&user_id=asdasdasd&user_id=aaaa
    let par = param.0.unwrap_or_default();
    let result = parse_query(&par);

    let mut user_data = UserParam {
        user_id: Vec::new(),
    };
    // Checking if key user_id are exists.
    if let Some(user_ids) = result.get("user_id") {
        // Appending to user_data with Extend func
        user_data
            .user_id
            .extend(user_ids.iter().map(|&s| s.to_string()))
    }
    dbg!(user_data.user_id);
    "asdadasd".to_string()
}
