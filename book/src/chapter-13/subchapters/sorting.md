### Sorting

Sorting lists, reversing and using a key.

**Python**

```python
names = ['taz', 'bugs', 'daffy']

# Lexicographical order
names.sort()

# Reversed lexicographical order
names.sort(reverse=True)

# Sort by length
names.sort(key=len)
```

**Rust**

```rust
fn main() {
    let mut names = ["taz", "bugs", "daffy"];

    // Lexicographical order
    names.sort();

    // Reversed lexicographical order
    names.sort_by(|a, b| b.cmp(a));

    // Sort by length
    names.sort_by_key(|a| a.len());
}
```
