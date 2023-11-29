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


    // Get cookie by number position, first from 0;
    let cookie_from_nr = setup_header.get_cookie_num(0).await;
    println!("Cookie Value from nr is: {:?}", cookie_from_nr.get_value());

    // Get List - all cookie names from site;
    let cookie_list = setup_header.list_cookies();
    println!("{:?}", cookie_list);

    // Get 2 Cookies from One header
    let resp = reqwest::get("http://webtest.5v.pl/cookie/").await.unwrap();
    let header = resp.headers();
    let cookie_header = easycookie::set_header(header.clone()).await;
    let first_cookie = cookie_header.get_cookie("random").await.get_value();
    let second_cookie = cookie_header.get_cookie("random2").await.get_value();
    println!("Cookies 1: {:?}, 2: {:?}", first_cookie, second_cookie);
}
