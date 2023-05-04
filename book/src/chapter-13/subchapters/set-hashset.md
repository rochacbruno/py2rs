### set / HashSet

Create a `set` (a hash of unique keys), add new keys and compute `intersection`, `difference` and `union`

**Python**

```python

# creating and populating
colors = set()
colors.add("red")
colors.add("green")
colors.add("blue")
colors.add("blue")

# using literal syntax
colors = {'red', 'green', 'blue', 'blue'}

# from an iterator
colors = set(['red', 'green', 'blue', 'blue'])


# deduplication
print(colors)  # {"blue", "green", "red"}

# operations
colors = {'red', 'green', 'blue', 'blue'}
flag_colors = {"red", "black"}

# difference
colors.difference(flag_colors)  # {'blue', 'green'}

# symmetric difference
colors.symmetric_difference(flag_colors)  # {'black', 'blue', 'green'}

# intersection
colors.intersection(flag_colors)  # {'red'}

# union
colors.union(flag_colors)  # {'black', 'blue', 'green', 'red'}

```

**Rust**

```rust
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {

    // creating and populating - type inference
    let mut colors = HashSet::new();
    colors.insert("red");
    colors.insert("green");
    colors.insert("blue");
    colors.insert("blue");

    // from an iterator - explicit type
    let mut colors: HashSet<&str> = HashSet::from_iter(vec!["red", "green", "blue", "blue"]);

    // deduplication
    println!("{:?}", colors); // {"blue", "green", "red"}

    // Operations
    let mut colors: HashSet<&str> = HashSet::from_iter(vec!["red", "green", "blue", "blue"]);
    let mut flag_colors: HashSet<&str> = HashSet::from_iter(vec!["red", "black"]);

    // difference
    colors.difference(&flag_colors); // ["green", "blue"]

    // symmetric difference
    colors.symmetric_difference(&flag_colors); // ["blue", "green", "black"]

    // intersection
    colors.intersection(&flag_colors); // ["red"]

    // union
    colors.union(&flag_colors); // ["red", "blue", "green", "black"]
}
```

or syntax sugared using [maplit](https://crates.io/crates/maplit) crate

```rust
#[macro_use] extern crate maplit;

let colors = hashset!{"red", "green", "blue", "blue"};
```