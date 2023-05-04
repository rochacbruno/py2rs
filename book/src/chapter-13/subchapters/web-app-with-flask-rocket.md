### Web app with Flask / Rocket

**Python**

```python
from flask import Flask

app = Flask(__name__)


@app.route('/')
def index():
    return 'Hello Python'


if __name__ == '__main__':
    app.run(port=8080)
```

**Rust**

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello Rust"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
```