### Installing new libraries/crates

**Python**

```bash
$ pip install foo 
```


**Rust**

```bash
$ cargo install foo
```

---

# running--compiling

**Python**

```bash
$ python my_python_program.py 
```


**Rust**

```bash
$ cargo run
```

In Rust, there is a `--release` flag that allows for more compile time optimization to be done, but it will take longer to compile

```bash
$ cargo run --release
```