### while and for loops

Looping until a condition is met or over an iterable object.

**Python**

```python
# While loop

counter = 0
while counter < 10:
    print(counter)
    counter += 1

# infinite while loop
while True:
    print("loop Forever!")

# infinite while loop with break
counter = 0
while True:
    print(counter)
    counter += 1
    if counter >= 10:
        break


# while loop with continue
counter = 0
while True:
    counter += 1
    if counter == 5:
        continue
    print(counter)
    if counter >= 10:
        break

# For loop over a list
for color in ["red", "green", "blue"]:
    print(color)

# Enumerating indexes
for  i, color in enumerate(["red", "green", "blue"]):
    print(f"{color} at index {i}")

# For in a range
for number in range(0, 100):
    print(number)  # from 0 to 99

```

**Rust**

```rust
fn main() {

    // While loop
    let mut counter = 0;
    while counter < 10 {
        println!("{}", counter);
        counter += 1;
    }

    // infinite while loop
    loop {
        println!("Loop forever!");
    }

    // infinite while loop with break
    let mut counter = 0;
    loop {
        println!("{}", counter);
        counter += 1;
        if counter >= 10 { break; }
    }

    // infinite while loop with continue
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 { continue; }
        println!("{}", counter);
        if counter >= 10 { break; }
    }

    // for loop over a list
    for color in ["red", "green", "blue"].iter() {
        println!("{}", color);
    }

    // Enumerating indexes
    for (i, color) in ["red", "green", "blue"].iter().enumerate() {
        println!("{} at index {}", color, i);
    }

    // for in a range
    for number in 0..100 {
        println!("{}", number);  // from 0 to 99
    }
}
```

#### Loop Labels

**Rust** has a looping feature which is not present on Python: **Loop labels**

```rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
}
```