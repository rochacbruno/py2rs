### Dict/Map

Create new dictionaries (hash maps), adding new keys and values, changing values, getting by key, checking if a key is containing, etc.

**Python**

```python

# Creating a new dict and populating it
ages = {}
ages['daffy'] = 80
ages['bugs'] = 79
ages['taz'] = 63

# or doing the same using a for loop
ages = {}
for name, age in [("daffy", 80), ("bugs", 79), ("taz", 63)]:
    ages[name] = age

# or initializing from a list
ages = dict([("daffy", 80), ("bugs", 79), ("taz", 63)])

# or passing key values on creation
ages = {  # Ages for 2017
    'daffy': 80,
    'bugs': 79,
    'taz': 63,
}

ages['elmer'] = 80
print(ages['bugs'])  # 79
print('bugs' in ages)  # True

del ages['taz']

for name in ages:  # Keys
    print(name)

for name, age in ages.items():  # Keys & values
    print('{} is {} years old'.format(name, age))
```

**Rust**

```rust
use std::iter::FromIterator;
use std::collections::HashMap;

fn main() {

    // Creating a new HashMap and populating it
    let mut ages = HashMap::new();  // Ages for 2017
    ages.insert("daffy", 80);
    ages.insert("bugs", 79);
    ages.insert("taz", 63);

    // or doing the same using a loop
    let mut ages = HashMap::new();
    for &(name, age) in [("daffy", 80), ("bugs", 79), ("taz", 63)].iter() {
        // For non-Copy data, remove & and use iter().clone()
        ages.insert(name, age);
    }

    // or initializing from Array
    let mut ages: HashMap<&str, i32> =  // Ages for 2017
        [("daffy", 80), 
         ("bugs", 79), 
         ("taz", 63)]
        .iter().cloned().collect();

    // or initializing from Vec (Iterator)
    let mut ages: HashMap<&str, i32> =  // Ages for 2017
        HashMap::from_iter(
            vec![
               ("daffy", 80),
               ("bugs", 79),
               ("taz", 63)
            ]
        );

    ages.insert("elmer", 80);
    println!("{}", ages["bugs"]);  // 79
    println!("{}", ages.contains_key("bugs")); // true
    ages.remove("taz");


    for name in ages.keys() {  // Keys
      println!("{}", name);
    }

    for (name, age) in &ages {  // Keys & values
      println!("{} is {} years old", name, age);
    }

}
```

#### Pythonic alternative to dict/map in Rust

You can use the [maplit](https://crates.io/crates/maplit) crate to load `hashmap!` macro to have an efficient sugared (a.k.a Pythonic) syntax!

```toml
# Cargo.toml
[dependencies]
maplit = "*"
```

then

```rust
#[macro_use] extern crate maplit;

let map = hashmap!{
    "daffy" => 80,
    "bugs" => 79,
    "taz" => 63,
};
```
