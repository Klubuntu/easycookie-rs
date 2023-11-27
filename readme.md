# easycookie-rs
Easy method for get cookie from header

### Requirements
- Rust
- reqwest - https://crates.io/crates/reqwest

### How to usage ?
```rust
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