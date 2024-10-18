### Operator overloading

Python and Rust allows the overload of the behavior of operators such as `+, -, /, *, %`

In both examples the operator `+` (add) is being overloaded to implement the Rock, Paper, Scissors game.

Python implementation uses its Protocol Oriented Programming, by looking for the `__add__` method, which is part of the structural **Summable** protocol.

Rust does the same, but using trait implementation, in this case we implement `ops::Add<T> for T`

Both are, despite the syntax differences, very similar on the concept.

**Python**

[playground]([https://repl.it/repls/uniquelinkhere](https://trinket.io/python3/9d63ebc8f966?runOption=run))

```python
from enum import Enum
from typing import Self


class Object(Enum):
    Rock = 1
    Paper = 2
    Scissors = 3

    def __add__(self, other: Self) -> Self:
        match self:
            case Object.Rock if other is Object.Scissors:
                return self
            case Object.Scissors if other is Object.Paper:
                return self
            case Object.Paper if other is Object.Rock:
                return self
            case _:
                return other


if __name__ == "__main__":
    assert Object.Rock + Object.Scissors is Object.Rock
    assert Object.Scissors + Object.Paper is Object.Scissors
    assert Object.Paper + Object.Rock is Object.Paper
    assert Object.Rock + Object.Rock is Object.Rock
    assert Object.Paper + Object.Paper is Object.Paper
    assert Object.Scissors + Object.Scissors is Object.Scissors

```

**Rust**

[playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=b19b1411126d3963e74fcd1deff67d1b)

```rust
use std::ops;

#[derive(Debug, PartialEq)]
enum Object {
    Rock,
    Paper,
    Scissors,
}

impl ops::Add<Object> for Object {
    type Output = Object;
    fn add(self, other: Object) -> Object {
        match self {
            Object::Rock if other == Object::Scissors => self,
            Object::Scissors if other == Object::Paper => self,
            Object::Paper if other == Object::Rock => self,
            _ => other,
        }
    }
}

fn main() {
   assert_eq!(Object::Rock + Object::Scissors, Object::Rock);
   assert_eq!(Object::Scissors + Object::Paper, Object::Scissors);
   assert_eq!(Object::Paper + Object::Rock, Object::Paper);
   assert_eq!(Object::Rock + Object::Rock, Object::Rock);
   assert_eq!(Object::Scissors + Object::Scissors, Object::Scissors);
   assert_eq!(Object::Paper + Object::Paper, Object::Paper);
}

```
