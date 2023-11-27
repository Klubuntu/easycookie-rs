# easycookie-rs
Easy method for get cookie from header

### Requirements
- Rust
- reqwest - https://crates.io/crates/reqwest

### Install
Run the following Cargo command in your project directory:


`cargo add easycookie`

Or add the following line to your Cargo.toml:

`easycookie = "1.0.0"`

### How to usage ?
```rust
use easycookie;
use reqwest;
let example_request = reqwest::get(your_url).await.unwrap();
let headers = example_request.headers();
```

### Examples
```rust
// Method 1
let cookie_value = easycookie::get_cookie(headers.clone(), "random").await;
println!("Cookie Value is: {:?}", cookie_value.get_value());
```
```rust
// Method 2
let setup_header = easycookie::set_header(headers.clone()).await;
let get_new_cookie = setup_header.get_cookie("random2").await;
println!("Cookie Value is: {:?}", get_new_cookie.get_value());
```

:star: Thank you for usage

Pull Requests are welcome 
