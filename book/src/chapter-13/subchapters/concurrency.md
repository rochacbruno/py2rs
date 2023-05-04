### Concurrency

**Python**

```python
thr = Thread(target=add, args=(1, 2), daemon=True)
thr.start()
```

**Rust**

```rust
use std::thread;

thread::spawn(|| {
        add(5,5);
    });

```