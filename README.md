# py2rs
## From Python into Rust 

```rust
let x = Rust::from("Python");
```

A quick reference guide for the **Pythonista** in process of becoming a **Rustacean**. 



### Monty Python - Season 3 - Episode 49

[The sketch](http://montypython.50webs.com/scripts/Series_3/49.htm)

```
Mrs. Jalin: George.
Mr. Jalin: Yes, Gladys.
Mrs. Jalin: There's a man at the door with a moustache.
Mr. Jalin: Tell him I've already got one. (Mrs. Jalin hits him hard with a newspaper) 
          All right, all right. What's he want then?
Mrs. Jalin: He says do we want a documentary on crustaceans.
Mr. Jalin: Crustaceans!
Mrs. Jalin: Yes.
Mr. Jalin: What's he mean, crustaceans?
Mrs. Jalin: CRUSTACEANS!! GASTROPODS! LAMELLIBRANCHS! CEPHALOPODS!
...
Ok...watch it later... let's learn some Rust now...
```

## TOC

- [Getting Started with Rust](https://github.com/rochacbruno/py2rs#getting-started-with-rust)
- [Where to exercice](https://github.com/rochacbruno/py2rs#exercices)
- [Where to be informed on news and updates](https://github.com/rochacbruno/py2rs#getting-updated)
- [Interacting with Rustacean Communities](https://github.com/rochacbruno/py2rs#interact-with-other-rustaceans)
- [Additional learning Resources](https://github.com/rochacbruno/py2rs#additional-learning-resources)
- [Curious Facts](https://github.com/rochacbruno/py2rs#facts)
- [Glossary of terms](https://github.com/rochacbruno/py2rs#glossary-of-terms)
- [General fact comparison](https://github.com/rochacbruno/py2rs#general)
- [Environment Tools](https://github.com/rochacbruno/py2rs#environment-tools)
- [Libraries and Frameworks](https://github.com/rochacbruno/py2rs#libraries-and-frameworks)
- [Applications](https://github.com/rochacbruno/py2rs#applications)
- [Code comparison Python X Rust implementations](https://github.com/rochacbruno/py2rs#show-me-the-code)
- [Credits](https://github.com/rochacbruno/py2rs#credits)



## Getting Started with Rust

Assuming you already know what is [**Rust**](http://rust-lang.org) and already 
decided to start learning it. Here are some steps for you to follow:

1) Take a tour of **Rust Syntax** and **Coding Style**  
   https://learnxinyminutes.com/docs/rust/
2) Watch some **screencasts** to get basics of **Ownership &Borrowing** concept  
   http://intorust.com/
3) Follow this set of runnable examples to understand how everything fit together  
   https://rustbyexample.com
4) Now it is time to read your first book, you can pick:  
    * **Rust Essentials**  
      > A good introduction to Rust language in a more `superficial` approach which results
      > in a very pleasant and easy reading, recommended even for those who are not experienced
      > with low level systems languages.  
        * Paparback and e-book available on Packt Publisher  
          https://www.packtpub.com/application-development/rust-essentials-second-edition

    * **TRPL** (The Rust Programming language Book)   
      > A complete Guide to Rust Language  
      > https://doc.rust-lang.org/book/   
        * Official Book  
        * Free to read online  
        * Available as papaerback or e-book (buy at Amazon)
5) Read some real examples
    * **Rust Cookbook**  
     https://rust-lang-nursery.github.io/rust-cookbook/
     This Rust Cookbook is a collection of simple examples that demonstrate good 
     practices to accomplish common programming tasks, using the crates of the Rust ecosystem.
    * **Anthology**  
    https://github.com/brson/rust-anthology/blob/master/master-list.md 
6) Patterns and Good Practices  
   * **Rust Patterns** https://github.com/rust-unofficial/patterns
   * **API Guidelines** https://rust-lang-nursery.github.io/api-guidelines/



## Exercices

Time to put your new knowledge in action solving some exercices.

1) **Exercism.io**  
   Register a new account on exercism.io (using github auth)  
   Install exercism command line client on your computer  
   Solve some exercices:  http://www.exercism.io/languages/rust/about 

2) **Rust Playground**  
   Run Live Rust Code in the browser with https://play.rust-lang.org/

## Getting updated

Now I assume your are addicted to **Rust** and you want to be updated about averything 
around it, here are some good links to follow.

1) This Week in Rust Newsletter  
   https://this-week-in-rust.org/   
   https://twitter.com/thisweekinrust 
2) Reddit  
   http://reddit.com/r/rust   (serious sub-reddit)  
   http://reddit.com/r/rustjerk (almost memes only)  
3) Official Twitter  
  https://twitter.com/rustlang

## Interact with other Rustaceans

> Don't be afraid, the Rustaceans are a very receptive species and are cozy with the Pythonistas.

Community links: https://www.rust-lang.org/en-US/community.html

### Local

- Brazil
    * General
        * Youtube :  http://bit.ly/canalrustbr
        * Telegram: https://t.me/rustlangbr
    * Rust BH
        * https://t.me/rustbh

- Add your country/city here, send a Pull Request.

## Additional learning resources

- Rust Learning https://github.com/ctjhoa/rust-learning
- Rust Guidelines (WIP) https://aturon.github.io/
- Rust Design Patterns and Idioms https://github.com/rust-unofficial/patterns/
- Idiomatic Rust https://github.com/mre/idiomatic-rust
- GTK Rust Tutorial https://mmstick.github.io/gtkrs-tutorials/
- Effective use of iterators http://hermanradtke.com/2015/06/22/effectively-using-iterators-in-rust.html
- Red Hat Developers: Speed up Your Python with Rust https://developers.redhat.com/blog/2017/11/16/speed-python-using-rust/

## Facts

- The language is named **Rust** because "rust is as close to the bare metal as you can get.",   
  in metal theory **rust** is the chemical layer closest to bare metal.
- The Rust `trifecta` is 1) **Memory Safe**, 2) **Fast** 3) **Concurrent**
- Rust can be used for web development http://www.arewewebyet.org/
- Rust can be used for Gaming Development http://arewegameyet.com/
- Rust can be used for Machine Learning http://www.arewelearningyet.com/ 
- Lots of IDEs and Editors supports Rust https://areweideyet.com/  
  (VSCode is known to have the better support by now https://marketplace.visualstudio.com/items?itemName=kalitaalexey.vscode-rust)
- Rust packages are called `Crates` and are installed by `Cargo` explore them at http://crates.io
- In Rust there is no **class** but **Structs**, **Enums**, **Traits**, **functions** and **macros!**
- The Rust mascot (unofficial) is called **Ferris** and it is a **crab** http://www.rustacean.net/  
  (There is no record of the official reason about being a crab, the reasonable history is that it was inspired by the **Rusty Crab** 
   a common species of crab and also a name of a famous restaurant.)

> **More facts?** send a question here or send a Pull Request adding an interest fact to this list.

![ferris](http://www.rustacean.net/more-crabby-things/animated-ferris.gif)

## Glossary of terms

| Term      | Definition                                        |
| --------- | ------------------------------------------------- |
| crate     | A rust distributable **package**                  |
| ferris    | The unofficial Crab Mascot                        |
| Rustacean | The Rust programmer or evangelist or enthusiastic |
| nightly   | The unstable toolchain of the Rust compiler       |
| impl      | Implementation                                    |

# py2rs
## From Python into Rust 

```rust
let x = Rust::from("Python");
```

A quick reference guide for the **Pythonista** in process of becoming a **Rustacean**. 


## General
| Python                                                                       | definition                         | Rust                                                                      |
| ---------------------------------------------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------- |
| [PEP8]()                                                                     | Guidelines and conventions         | [RustAPI Guidelines](https://rust-lang-nursery.github.io/api-guidelines/) |
| [PEPS]()                                                                     | Enhancement Proposals / RFC        | [Rust RFCs]()                                                             |
| [PSF]()                                                                      | Organization / Foundation          | [Mozilla Research]()                                                      |
| [PyCon]()                                                                    | Main Conference                    | [RustConf]()                                                              |
| [Guido Van Rossum]()                                                         | Creator                            | [Graydon Hoare]()                                                         |
| 1989                                                                         | First appeared                     | 2010                                                                      |
| 1991                                                                         | First Release                      | 2012                                                                      |
| [PSF]()                                                                      | License                            | Apache 2.0 and MIT                                                        |
| C                                                                            | Implemented in                     | Rust                                                                      |
| .py, .pyw, .pyc                                                              | File Extensions                    | .rs, .rlib                                                                |
| http://github.com/python/cpython                                             | Repository                         | https://github.com/rust-lang/rust                                         |
| [Pyladies](), [AfroPython]()                                                 | Diversity and Inclusion initiative | [RustBridge](https://rustbridge.github.io/)                               |
| [comp.lang.Python](https://groups.google.com/forum/#!forum/comp.lang.python) | Official Users Forum               | [users.rust-lang.org](https://users.rust-lang.org)                        |

## Environment Tools

| Python                                                    | Definition                              | Rust                                                                       |
| --------------------------------------------------------- | --------------------------------------- | -------------------------------------------------------------------------- |
| [PyPI]()                                                  | Library Repositoty                      | [Crates.io](http://crates.io)                                              |
| [pip]()                                                   | Library installation                    | [Cargo]()                                                                  |
| [setuptools]()                                            | Library distribution                    | [Cargo]()                                                                  |
| [pbr](https://docs.openstack.org/pbr/latest/)             | Library distribution                    | [Cargo]()                                                                  |
| [pipenv]()                                                | Dependency manager                      | [Cargo]()                                                                  |
| [twine]()                                                 | Package uploader                        | [Cargo]() and [Semantic](https://github.com/semantic-rs/semantic-rs)       |
| [virtualenv]()                                            | Isolated environments                   | [Cargo]()                                                                  |
| [pyinstaller](https://github.com/pyinstaller/pyinstaller) | Generate Standalone Executables         | [Cargo]()                                                                  |
| [pyenv]()                                                 | Install and manage versions of language | [rustup]()                                                                 |
| [sphinx]()                                                | Generate documentation from code        | [rustdoc]() and [cargo]()                                                  |
| [python]()                                                | Interpreter / Compiler                  | [rustc]() and [cargo]()                                                    |
| [ipython]()                                               | REPL                                    | [?]()                                                                      |
| [ipdb   ]()                                               | Debugger                                | [rust-gdb](https://github.com/rust-lang/rust/blob/master/src/etc/rust-gdb) |

## Libraries and Frameworks

| Python                                             | Definition                       | Rust                                                                                                  |
| -------------------------------------------------- | -------------------------------- | ----------------------------------------------------------------------------------------------------- |
| [urllib]()                                         | HTTP calls                       | [hyper]()                                                                                             |
| [requests]()                                       | simplified HTTp calls            | [reqwest](https://github.com/seanmonstar/reqwest)                                                     |
| [json]()                                           | JSON Parsing loading and dumping | [serde](https://github.com/serde-rs/serde)                                                            |
| [pyYAML]()                                         | YAML Parsing loading and dumping | [serde](https://github.com/serde-rs/serde)                                                            |
| [lxml]()                                           | XML Parsing loading and dumping  | [RustyXML](https://github.com/Florob/RustyXML)                                                        |
| [csv]()                                            | CSV parsing                      | [rust-csv](https://github.com/BurntSushi/rust-csv)                                                    |
| [datetime]() & [Dateutils]()                       | Date & time                      | [Chrono](https://github.com/chronotope/chrono)                                                        |
| [click]() and [argparse]()                         | CLI Framework                    | [clap](https://github.com/kbknapp/clap-rs)                                                            |
| [docopt]()                                         | CLi Framework                    | [docopt](https://github.com/docopt/docopt.rs)                                                         |
| [re]()                                             | Regular Expressions              | [regex](https://github.com/rust-lang/regex)                                                           |
| [subprocess]()                                     | Run external commands            | [crossbeam](https://github.com/crossbeam-rs/crossbeam) and [Rayon](https://github.com/rayon-rs/rayon) |
| [logging]()                                        | Logging                          | [log](https://github.com/rust-lang-nursery/log)                                                       |
| [Pathlib]()                                        | Path manipulation                | [fs]() and [fs_extra](https://github.com/webdesus/fs_extra)                                           |
| [crypto]()                                         | crytography                      | [crypto](https://github.com/DaGenix/rust-crypto)                                                      |
| [pickle]()                                         | Object Serialization             | [RON](https://github.com/ron-rs/ron)                                                                  |
| [heapq]()                                          | Heap Queue                       | [?]()                                                                                                 |
| [bottle]()                                         | Minimal Web Framework            | [Iron](https://github.com/iron/iron)                                                                  |
| [flask]()                                          | Web Framework                    | [Rocket](https://github.com/SergioBenitez/Rocket)                                                     |
| [django]()                                         | Full Stack Web Framrwork         | [Gotham](https://github.com/gotham-rs/gotham)                                                         |
| [SQL Alchemy]()                                    | Relational Database ORM          | [Diesel](https://github.com/diesel-rs/diesel)                                                         |
| [Pymongo]()                                        | Mongo DB driver                  | [mongodb](https://crates.io/keywords/mongodb)                                                         |
| [Jinja 2]()                                        | Template Engine                  | [Tera](https://github.com/Keats/tera)                                                                 |
| [pygtk](http://www.pygtk.org/)                     | GTk desktop development          | [gtk](https://github.com/gtk-rs/gtk)                                                                  |
| [pyside](http://www.pyside.org/)                   | QT desktop development           | [rust-qt](https://github.com/rust-qt)                                                                 |
| [pygame]()                                         | 2D UI library / gaming           | [Conrod](https://github.com/PistonDevelopers/conrod/) & [Piston](http://www.piston.rs/)               |
| [unitest2]()                                       | Test framework                   | [Builtin]()                                                                                           |
| [nose]()                                           | Test Runner                      | [Cargo]()                                                                                             |
| [pytest]()                                         | Testing Framework and Runner     | [Polish](https://github.com/AlKass/polish)                                                            |
| [Flake8]()                                         | Linter                           | [Clippy](https://github.com/rust-lang-nursery/rust-clippy)                                            |
| [autopep8]()                                       | Auto formatter                   | [rustfmt]()                                                                                           |
| [twisted]()                                        | Network application framework    | [libpnet](https://github.com/libpnet/libpnet)                                                         |
| [asyncIO]()                                        | Async application framework      | [Tokio](https://github.com/tokio-rs/tokio) and [futures](https://github.com/alexcrichton/futures-rs)  |
| [PIL]()                                            | Image Manipulation               | [Image](https://github.com/PistonDevelopers/image)                                                    |
| [Beautiful Soup]()                                 | HTML Parser                      | [html5ever](https://github.com/servo/html5ever)                                                       |
| [Hypothesis](http://hypothesis.works/)             | Data Driven test framework       | [proptest](https://github.com/altsysrq/proptest)                                                      |
| [mock]()                                           | Test Mocking                     | [Mockito](https://github.com/lipanski/mockito)                                                        |
| [bioPython]()                                      | Bioinformathics libraries        | [Rust Bio](https://github.com/rust-bio)                                                               |
| [Dynaconf](http://github.com/rochacbruno/dynaconf) | Config management                | [Config](https://github.com/mehcode/config-rs)                                                        |
| [Itertools]()                                      | Data Structure Iteration         | [Rust Itertools](https://github.com/bluss/rust-itertools)                                             |
| [Geopython]()                                      | geo Spatial Data                 | [Geo Rust](https://github.com/georust)                                                                |
| [ScikitLearn]()                                    | Machine Learning                 | [rusty-machine](https://github.com/AtheMathmo/rusty-machine)                                          |
| [mistune]()                                        | Markdown / Common Mark Parser    | [cmark](https://github.com/google/pulldown-cmark)                                                     |
| [celery]()                                         | Distributed Computation          | [Antimony](https://github.com/antimonyproject/antimony)                                               |
| [boto]()                                           | AWS clients                      | [rusoto](https://github.com/rusoto/rusoto)                                                            |
| [AstroPy]()                                        | Astronomy                        | [atro-rust](https://github.com/saurvs/astro-rust)                                                     |
| [Numpy]()                                        | Numeric                          | [?]()                                                     |
 

## Applications

| Python                             | Definition                                       | Rust                                                   |
| ---------------------------------- | ------------------------------------------------ | ------------------------------------------------------ |
| [Pelican]()                        | Static Site generator                            | [Cobalt](https://github.com/cobalt-org/cobalt.rs)      |
| [ansible]()                        | Infra Orchestration                              | [realize]()                                            |
| [mkdocs]()                         | Generate documentation and e-books from Markdown | [mdBook]()                                             |
| [locust]()                         | HTTP load test                                   | [drill](https://github.com/fcsonline/drill)            |
| [Nameko]()                         | Microservices Framework                          | [fractalide](https://github.com/fractalide/fractalide) |
| [Quokka](http://quokkaproject.org) | CMS                                              | [NIckel CMS](https://github.com/irony-rust/nickel-cms) |


## Show me The code

From **Python** to **Rust** by examples  

> You can  copy-paste and run the **Rust** examples in https://play.rust-lang.org/ and **Python** in https://repl.it/languages/python3 


### Creating a new project

**Python**

```bash
$ mkdir my_python_program.py 
```


**Rust**

```bash
$ cargo new my-rust-program
```

---

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

### Running / Compiling

**Python**

```bash
$ python my_python_program.py 
```


**Rust**

```bash
$ cargo run
```

---

### Hello World

**Python**

```python
if __name__ == "__main__":
    print("Hello, World")
```

**Rust**

```rust
fn main() {
  println!("Hello, World");
}
```

---

### Types and Declarations

Create new objects, values on basic primitive types and also data structures.

**Python**

```python
age = 80
name = 'daffy'
weight = 62.3
loons = ['bugs', 'daffy', 'taz']
ages = {  # Correct for 2017
    'daffy': 80,
    'bugs': 79,
    'taz': 63,
}
```

**Rust**

```rust
use std::collections::HashMap;

fn main() {
    let age = 80;
    let name = "daffy";
    let weight = 62.3;
    let mut loons = vec!["bugs", "daffy", "taz"];

    let mut ages = HashMap::new();   // Correct for 2017
    ages.insert("daffy", 80);
    ages.insert("bugs", 79);
    ages.insert("taz", 63);
}

```
---

### Define a function

Defining a function that takes 2 integer arguments and returns its sum.

**Python**

```python
def add(a, b):
    "Adds a to b"""
    return a + b
```

**Rust**

```rust
// Adds a to b
fn add(a: i32, b: i32) -> i32 {
  a + b
}
```

---

### List/Slice

Creating a list, adding new elements, gettings its length, slicing by index, itarating using for loop and iterating with enumerator.

**Python**

```python
names = ['bugs', 'taz', 'tweety']
print(names[0])  # bugs
names.append('elmer')
print(len(names))  # 4
print(names[2:])  # ['tweety', 'elmer']

for name in names:
    print(name)

for i, name in enumerate(names):
    print('{} at {}'.format(name, i))
```

**Rust**

```rust
fn main() {
    let mut names = vec!["bugs", "taz", "tweety"];
    println!("{}", names[0]);  // bugs
    names.push("elmer");
    println!("{}", names.len());  // 4
    println!("{:?}", &names[2..]);  // ["tweety", "elmer"]

    for name in &names {
        println!("{}", name);
    }

    for (i, name) in names.iter().enumerate() {
        println!("{} at {}", i, name);
    }
}
```
---

### Dict/Map

Create new dictionaries (hash maps), adding new keys and values, changing values, getting by key, checking if a key is containing, etc.

**Python**

```python
ages = {  # Correct for 2017
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
fn main() {

    let mut ages = HashMap::new();  // correct for 2017
    ages.insert("daffy", 80);
    ages.insert("bugs", 79);
    ages.insert("taz", 63);

    // or initializing from an Array:
    let mut ages: HashMap<&str, i32> =  // correct for 2017
        [("daffy", 80), 
         ("bugs", 79), 
         ("taz", 63)]
        .iter().cloned().collect();

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

---

### While loop

Loop a range while it mets some condition.

**Python**

```python
# Largest Fibonacci under 10,000
a, b = 1, 1
while b < 10000:
    a, b = b, a + b
```

**Rust**

```rust
```

---

### Files

Read a text file and iterate its lines printing the content, properly close the file at the end.

**Python**

```python
with open('song.txt') as fp:
    #  Iterate over lines
    for line in fp:
        print(line.strip())
```

**Rust**

```rust
```

---

### Exceptions/Return Error

Expecting for exceptions and identifying errors.

**Python**

```python
def div(a, b):
    if b == 0:
        raise ValueError("b can't be 0")
    return a / b

# ...

try:
    div(1, 0)
except ValueError:
    print('OK')
```

**Rust**

```rust
```

---

### Concurrency

**Python**

```python
thr = Thread(target=add, args=(1, 2), daemon=True)
thr.start()
```

**Rust**

```rust
```

---

### Communicating between threads

Managing data context between threads.

**Python**

```python
from queue import Queue
queue = Queue()
# ...
# Send message from a thread
queue.put(353)


# ...
# Get message to a thread
val = queue.get()
```

**Rust**

```rust
```

---

### Sorting

Sorting lists, reversing and using a key.

**Python**

```python
names = ['taz', 'bugs', 'daffy']

# Lexicographical order
names.sort()

# Reversed lexicographical order
names.sort(reverse=True)

# Sort by length
names.sort(key=len)
```

**Rust**

```rust
```

---

### Web app with Flask / Rocket

**Python**

```python
from flask import Flask

app = Flask(__name__)


@app.route('/')
def index():
    return 'Hello Python'


if __name__ == '__main__':
    app.run(port=8080)
```

**Rust**

```rust
#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello Rust"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
```

---

### HTTP Request with error handling

**Python**

```python
import json
from urlib2 import urlopen

url = 'https://httpbin.org/ip'
try:
    fp = urlopen(url)
except HTTPError as err:
    msg = 'error: cannot get {!r} - {}'.format(url, err)
    raise SystemExit(msg)

try:
    reply = json.load(fp)
except ValueError as err:
    msg = 'error: cannot decode reply - {}'.format(err)
    raise SystemExit(msg)

print(reply['origin'])

```

**Rust**

```rust
```

---

### Encode and Decode JSON

**Python**

```python
data = '''{
    "name": "bugs",
    "age": 76
}'''
obj = json.loads(data)

json.dump(obj, stdout)
```

**Rust**

```rust
```


---


### Print Object for Debug/Log 

**Python**

```python
daffy = Actor(
    name='Daffy',
    age=80,
)
print('{!r}'.format(daffy))
```

**Rust**

```rust
```

---

### Object Orientation

**Python**

```python
class Cat:
    def __init__(self, name):
        self.name = name

    def greet(self, other):
        print("Meow {}, I'm {}".format(other, self.name))

# ...

grumy = Cat('Grumpy')
grumy.greet('Grafield')
```

**Rust**

```rust
```

---

### Template for new examples

Explanation comes here.

**Python**

```python
# python code goes here
```

**Rust**

```rust
// rust code goes here
```

---

## Credits

Created by [Bruno Rocha](http://about.me/rochacbruno) [@rochacbruno](http://github.com/rochacbruno) inspired by https://www.353.solutions/py2go/index.html

With contributions by:

- Send a PR and include your name and links
