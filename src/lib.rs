use reqwest::header::HeaderMap;
use serde::__private::fmt::Debug;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct HeaderCookie{
    header: HeaderMap
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CookieValue {
    value: String
}

impl HeaderCookie{
    pub fn get_header(&self) -> &HeaderMap {
        &self.header
    }
    pub async fn get_cookie(&self, _name: &str) -> CookieValue{
        get_cookie(self.header.clone(), _name).await
    }
}

impl CookieValue {
    pub fn get_value(&self) -> &String {
        &self.value
    }
}

pub async fn set_header(_header: HeaderMap) -> HeaderCookie{
    HeaderCookie{header: _header}
}

pub async fn get_cookie(_header: HeaderMap, _name: &str) -> CookieValue{
    let cookies = _header.get_all("set-cookie");
    for cookie in cookies {
        let str_cookie = cookie.to_str().unwrap();
        let parse_cookie = str_cookie.split(';').next();
        let _cookie_name = parse_cookie.and_then(|cookie| cookie.split("=").next()).unwrap();
        if _cookie_name == _name {
            let cookie_value = parse_cookie.and_then(|cookie| cookie.split("=").nth(1)).unwrap();
            return CookieValue{value: String::from(cookie_value)};
        }
    }
    CookieValue{value: String::from("")}
}
    
    