### Multithreaded HTTP Crawler

**Python**

Using [**requests**](http://pypi.org/project/requests)

```python
from concurrent.futures import ThreadPoolExecutor
import requests


URLS = (
    "https://httpbin.org/html",
    "https://httpbin.org/links/10/0",
    "https://httpbin.org/robots.txt",
    "https://httpbin.org/user-agent",
    "https://httpbin.org/links/10/0",
    "https://httpbin.org/robots.txt",
    "https://httpbin.org/xml",
    "https://httpbin.org/redirect/1",
    "https://httpbin.org/redirect/2",
    "https://httpbin.org/cookies",
    "https://httpbin.org/basic-auth/user/passwd",
    "https://httpbin.org/gzip",
)


def crawl_worker(url):
    try:
        print(f"Response of url: {url} is {requests.get(url).status_code}")
    except Exception:
        print("Failed to get url.")


if __name__ == "__main__":
    with ThreadPoolExecutor() as executor:
        executor.map(crawl_worker, URLS)
```

**Rust**

using [**reqwest**](https://crates.io/crates/reqwest)

```rust
extern crate reqwest;
use std::thread;


fn crawl_worker(url: &str) {
    let parsed_url = reqwest::Url::parse(url).expect("Bad url format.");
    let response = reqwest::get(parsed_url).expect("Failed to get url.");
    println!("Response of url: {} is {:?}", url, response.status().to_string());
}


fn main() {
    let urls = vec![
        "https://httpbin.org/html",
        "https://httpbin.org/links/10/0",
        "https://httpbin.org/robots.txt",
        "https://httpbin.org/user-agent",
        "https://httpbin.org/links/10/0",
        "https://httpbin.org/robots.txt",
        "https://httpbin.org/xml",
        "https://httpbin.org/redirect/1",
        "https://httpbin.org/redirect/2",
        "https://httpbin.org/cookies",
        "https://httpbin.org/basic-auth/user/passwd",
        "https://httpbin.org/gzip",
    ];
    let mut queue = vec![];

    for url in urls {
        queue.push(thread::spawn(move || {
            crawl_worker(url);
        }));
    }

    for job in queue {
        let _ = job.join();
    }
}
```