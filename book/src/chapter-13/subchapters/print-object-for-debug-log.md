### Print Object for Debug/Log 

Print formatted object debug information

**Python**

```python

class Actor:
    def __init__(self, name, age):
        self.name = name
        self.age = age

    def __repr__(self):
      return str(self.__dict__)

daffy = Actor(
    name='Daffy',
    age=80,
)

print('{!r}'.format(daffy))  # {'name': 'Daffy', 'age': 80}
```

**Rust**

```rust
#[derive(Debug)]
struct Actor {
    name: String,
    age: i32
}

fn main() {
    let daffy = Actor {name: "Daffy".into(), age: 80};
    println!("{:#?}", daffy);   // Actor {name: "Daffy", age: 80 }
}

```