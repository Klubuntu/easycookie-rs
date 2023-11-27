use easycookie;
use reqwest;

#[tokio::main]
async fn main() {
    let example_request = reqwest::get("http://webtest.5v.pl/cookie/").await.unwrap();
    let headers = example_request.headers();

    // Method 1
    let cookie_value = easycookie::get_cookie(headers.clone(), "random").await;
    println!("Cookie Value is: {:?}", cookie_value.get_value());

    // Method 2
    let setup_header = easycookie::set_header(headers.clone()).await;
    let get_new_cookie = setup_header.get_cookie("random2").await;
    println!("Cookie Value is: {:?}", get_new_cookie.get_value());

}
