### Files

Read a text file and iterate its lines printing the content, properly close the file at the end.

**Python**

```python
from pathlib import Path

with Path("/tmp/song.txt").open() as fp:
    #  Iterate over lines
    for line in fp:
        print(line.strip())
```

**Rust**

```rust
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::path::Path;


fn main () {
    let fp = File::open(Path::new("/tmp/song.txt")).unwrap();
    let file = BufReader::new(&fp);
    for line in file.lines() {
        //  Iterate over lines
        println!("{}", line.unwrap());
    }
}

```