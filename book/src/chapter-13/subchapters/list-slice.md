### List/Slice

Creating a list, adding new elements, gettings its length, slicing by index, iterating using for loop and iterating with enumerator.

**Python**

```python
names = ['bugs', 'taz', 'tweety']
print(names[0])  # bugs
names.append('elmer')
print(len(names))  # 4
print(names[2:])  # ['tweety', 'elmer']

for name in names:
    print(name)

for i, name in enumerate(names):
    print('{} at {}'.format(name, i))
```

**Rust**

```rust
fn main() {
    let mut names = vec!["bugs", "taz", "tweety"];
    println!("{}", names[0]);  // bugs
    names.push("elmer");
    println!("{}", names.len());  // 4
    println!("{:?}", &names[2..]);  // ["tweety", "elmer"]

    for name in &names {
        println!("{}", name);
    }

    for (i, name) in names.iter().enumerate() {
        println!("{} at {}", i, name);
    }
}
```

**.step_by()** is the equivalent for python's range/xrange step parameter.

python:
```py
for i in range(0,10,2):
   print(i) # 0, 2, 4, 6, 8
```

rust:
```rust
for i in (0..10).step_by(2) {
    println!("{}", i);  // 0, 2, 4, 6, 8
}
```