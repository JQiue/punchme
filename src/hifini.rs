use std::env;

use regex::Regex;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, COOKIE};

async fn get_sign_code() -> String {
  let cookie = env::var("HIFINI_COOKIE").expect("HIFINI_COOKIE 获取失败");
  let mut headers = HeaderMap::new();
  headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
  let response = reqwest::Client::new()
    .get("https://www.hifini.com/sg_sign.htm")
    .headers(headers)
    .send()
    .await
    .expect("Failed to send request");
  let _status = response.status();
  let text = response.text().await.expect("Failed to get the response text");
  let regex = Regex::new(r#"sign\s*=\s*"([^"]+)""#).unwrap();
  let code = regex.captures(text.as_str()).and_then(|caps| caps.get(1)).map(|m| m.as_str().to_string()).unwrap();
  code
}

pub async fn sign_in() {
  let sign_code = get_sign_code().await;
  let cookie = env::var("HIFINI_COOKIE").expect("HIFINI_COOKIE 获取失败");
  let client = reqwest::Client::new();
  let mut headers = HeaderMap::new();
  headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"));
  headers.insert(COOKIE, HeaderValue::from_str(&cookie).unwrap());
  headers.insert("X-Requested-With", HeaderValue::from_static("XMLHttpRequest"));
  let response = client
    .post("https://www.hifini.com/sg_sign.htm")
    .headers(headers)
    .body(format!("sign={}", sign_code))
    .send()
    .await
    .expect("Failed to send request");

  let status = response.status();
  let text = response.text().await.expect("Failed to get the response text");
  println!("{status}");
  println!("{text}");
}
