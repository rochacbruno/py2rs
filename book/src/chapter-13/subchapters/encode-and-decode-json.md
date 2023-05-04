### Encode and Decode JSON

**Python**

```python
import json

# Decode/Deserialize
data = '''{
    "name": "bugs",
    "age": 76
}'''

person = json.loads(data)

# Do things like with any other Python data structure
print(f"{person['name']} was born {person['age']} years ago")

# Encode/Serialize
serialized = json.dumps(obj)
print(f"The serialized value is: {serialized}")
```

**Rust**

```rust
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

fn main() {
    // Decode/Deserialize
    let data = r#"{"name": "bugs", "age": 76}"#;

    let p: Person = match serde_json::from_str(data) {
        Ok(person) => person,
        Err(e) => panic!("error: could not deserialize: {}", e),
    };

    // Do things just like with any other Rust data structure.
    println!("{} was born {} years ago.", p.name, p.age);

    // Encode/Serialize
    let serialized = serde_json::to_string(&p).unwrap();
    println!("The serialized value is: {}", serialized);
}

```