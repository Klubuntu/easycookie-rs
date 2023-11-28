use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
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
    pub async fn get_cookie_num(&self, _num: i64) -> CookieValue{
        get_cookie_num(self.header.clone(), _num).await
    }
    pub fn list_cookies(&self){
        list_cookies(self.header.clone())
    }
}

impl CookieValue {
    pub fn get_value(&self) -> &String {
        &self.value
    }
}

fn __parse_cookie(cookie: &HeaderValue) -> Option<&str>{
    let str_cookie = cookie.to_str().unwrap();
    let parse_cookie = str_cookie.split(';').next();
    return parse_cookie;
}

pub async fn set_header(_header: HeaderMap) -> HeaderCookie{
    HeaderCookie{header: _header}
}

pub async fn get_cookie(_header: HeaderMap, _name: &str) -> CookieValue{
    let cookies = _header.get_all("set-cookie");
    for cookie in cookies {
        let parsed_cookie = __parse_cookie(cookie);
        let _cookie_name = parsed_cookie.and_then(|cookie| cookie.split("=").next()).unwrap();
        if _cookie_name == _name {
            let cookie_value = parsed_cookie.and_then(|cookie| cookie.split("=").nth(1)).unwrap();
            return CookieValue{value: String::from(cookie_value)};
        }
    }
    CookieValue{value: String::from("")}
}

pub async fn get_cookie_num(_header: HeaderMap, _num: i64) -> CookieValue{
    let cookies = _header.get_all("set-cookie");
    let cookie = cookies.iter().nth(_num.try_into().unwrap()).unwrap();
    let parsed_cookie = __parse_cookie(cookie);
    let cookie_value = parsed_cookie.and_then(|cookie| cookie.split("=").nth(1)).unwrap();
    CookieValue{value: String::from(cookie_value)}
}

pub fn list_cookies(_header: HeaderMap){
    println!("[EasyCookie Lib] Printing the cookie list\n---------------------");
    let cookies = _header.get_all("set-cookie");
    for cookie in cookies {
        let parsed_cookie = __parse_cookie(cookie);
        let _cookie_name = parsed_cookie.and_then(|cookie| cookie.split("=").next()).unwrap();
        println!("{:?}", _cookie_name)
    }
    println!("---------------------");
}
