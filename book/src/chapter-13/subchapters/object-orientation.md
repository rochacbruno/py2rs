### Object Orientation

**Python**

[playground](https://repl.it/repls/SourPaltryHorseshoecrab)

```python
class Cat:
    def __init__(self, name):
        self.name = name

    def greet(self, other):
        print("Meow {}, I'm {}".format(other, self.name))

# ...

grumy = Cat('Grumpy')
grumy.greet('Garfield')  # Meow Garfield, I'm Grumpy
```

**Rust**

[playground](https://play.rust-lang.org/?gist=155c76fae5240e483858000e73e82f00&version=stable)

```rust
struct Cat {
    name: String
}

impl Cat {

    pub fn new<S>(name: S) -> Cat where S: Into<String> {
        Cat { name: name.into() }
    }
    
    pub fn greet<S: Into<String>>(&self, other:S) {
        println!("Meow {}, I'm {}", other.into(), self.name);
    }     
    
}

fn main() {
    let grumpy = Cat::new("Grumpy");
    grumpy.greet("Garfield");  // Meow Garfield, I'm Grumpy
}
```

> NOTE: In Rust, it is best to avoid `stringly types APIs` so in the above example it would be better if we do `let garfield = Cat::new("Garfield")` and then make `greet` to accept an instance of `Cat` as `other` argument. If you are interested [watch this](https://www.youtube.com/watch?v=0zOg8_B71gE).