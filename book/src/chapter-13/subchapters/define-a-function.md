### Define a function

Defining a function that takes 2 integer arguments and returns its sum.

**Python**

```python
def add(a, b):
    """Adds a to b"""
    return a + b
```
**Python with typing annotations**

It looks more similar to Rust.

```python
def add(a: int, b: int) -> int:
    """Adds a to b"""
    return a + b
```

**Rust**

```rust
/// Adds a to b
fn add(a: i32, b: i32) -> i32 {
  a + b
}
```