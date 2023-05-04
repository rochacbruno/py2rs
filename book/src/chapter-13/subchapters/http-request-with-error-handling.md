### HTTP Request with error handling

**Python**

Using [**requests**](http://pypi.org/project/requests)

```python
import requests

url = 'https://httpbin.org/ip'

try:
    resp = requests.get(url)
except HTTPError as err:
    msg = f"error: cannot get {url} - {err}"
    raise SystemExit(msg)

assert resp.status_code == 200

print(f"The response content is: {resp.content}")

```

**Rust**

using [**reqwest**](https://crates.io/crates/reqwest)

```rust
extern crate reqwest;
use std::io::Read;

fn main() {
    let url = "https://httpbin.org/ip";

    let mut resp = match reqwest::get(url) {
        Ok(response) => response,
        Err(e) => panic!("error: could not perform get request {}", e),
    };

    assert!(resp.status().is_success());

    let mut content = String::new();
    resp.read_to_string(&mut content).expect("valid UTF-8");

    println!("The response content is: {}", content);
}
```