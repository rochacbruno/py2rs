### Creating a new project

Create a new project with basic files, entry points, module initializer, dependency and installation artifacts.

**Python**

```python
$ mkdir {pyproject,pyproject/src}
$ touch {pyproject/src/{__init__.py,__main__.py,program.py},pyproject/{requirements.txt,setup.py}} 
$ echo "-e ." >> pyproject/requirements.txt
$ echo "from setuptools import setup" >> pyproject/setup.py
$ echo "setup(author=..., name=...)" >> pyproject/setup.py
```

**Rust**

```rust
$ cargo new my-rust-program
```