### Types and Declarations

Create new objects, values on basic primitive types and also data structures.

**Python**

```python
age = 80
name = 'daffy'
weight = 62.3
loons = ['bugs', 'daffy', 'taz']
ages = {  # Ages for 2017
    'daffy': 80,
    'bugs': 79,
    'taz': 63,
}
```

**Rust**

```rust
use std::collections::HashMap;

fn main() {
    let age = 80;
    let name = "daffy";
    let weight = 62.3;
    let mut loons = vec!["bugs", "daffy", "taz"];

    let mut ages = HashMap::new();  // Ages for 2017
    ages.insert("daffy", 80);
    ages.insert("bugs", 79);
    ages.insert("taz", 63);
}

```