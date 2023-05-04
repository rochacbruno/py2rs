### Exceptions/Return Error

Expecting for exceptions and identifying errors.

**Python**

```python
def div(a, b):
    if b == 0:
        raise ValueError("b can't be 0")
    return a / b

# ...

try:
    div(1, 0)
except ValueError:
    print('An error occurred!')
```

**Rust**

```rust
fn div(a: i32, b: i32) -> Result<i32, &'static str> {
    if b == 0 {
        Err("b can't be 0")
    } else {
        Ok(a / b)
    }
}

fn main() {
    match div(1, 0) {
        Ok(_) => {},
        Err(_) => println!("An error occurred!"),
    };
}
```