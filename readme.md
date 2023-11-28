# easycookie-rs
📌 Easy method for get cookie from header

#### Index
- [Loading Time](https://github.com/Klubuntu/easycookie-rs/blob/main/readme.md#loading-time)
- [Install](https://github.com/Klubuntu/easycookie-rs/blob/main/readme.md#install)
- [How to use ?](https://github.com/Klubuntu/easycookie-rs/blob/main/readme.md#how-to-use-)
- [Examples](https://github.com/Klubuntu/easycookie-rs/blob/main/readme.md#examples)
- [More Features](https://github.com/Klubuntu/easycookie-rs/blob/main/readme.md#other-features)
- [How to support ?](https://github.com/Klubuntu/easycookie-rs/blob/main/readme.md#support-project)

### Requirements
- Rust
- reqwest - https://crates.io/crates/reqwest

### Install
Run the following Cargo command in your project directory:


`cargo add easycookie`

Or add the following line to your Cargo.toml:

`easycookie = "1.0.2"`

### Loading Time
```bash
Cookie Value from nr is: "data1"

real    0m3.214s
user    0m1.582s
sys     0m1.040s


Cookie Value is: "data2"

real    0m2.867s
user    0m1.540s
sys     0m1.037s
```

### How to use ?
```rust
use easycookie;
use reqwest;
let example_request = reqwest::get(your_url).await.unwrap();
let headers = example_request.headers();
```

### Examples
```rust
// Method 1
let cookie_value = easycookie::get_cookie(headers.clone(), "cookie_name").await;
println!("Cookie Value is: {:?}", cookie_value.get_value());
```
```rust
// Method 2
let setup_header = easycookie::set_header(headers.clone()).await;
let get_new_cookie = setup_header.get_cookie("cookie_name").await;
println!("Cookie Value is: {:?}", get_new_cookie.get_value());
```

### Other Features
```rust
// Get cookie by number position, first from 0;
let cookie_from_nr = setup_header.get_cookie_num(0).await;
println!("Cookie Value from nr is: {:?}", cookie_from_nr.get_value());

// Get List - all cookie names from site;
let cookie_list = setup_header.list_cookies();
println!("{:?}", cookie_list);
```

### Support Project

:star: Thank you for usage

Pull Requests are welcome 
