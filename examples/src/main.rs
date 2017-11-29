use std::iter::FromIterator;
use std::collections::HashMap;

fn types_and_declarations() {
    let age = 80;
    let name = "daffy";
    let weight = 62.3;
    let mut loons = vec!["bugs", "daffy", "taz"];
    loons.push("foo");

    let mut ages = HashMap::new(); 
    ages.insert("daffy", 80);
    ages.insert("bugs", 79);
    ages.insert("taz", 63);
}

fn add(a: i32, b: i32) -> i32 {
  a + b
}

fn list_slice() {
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


fn dict_map() {
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
    let mut ages: HashMap<&str, i32> = 
        [("daffy", 80), 
         ("bugs", 79), 
         ("taz", 63)]
        .iter().cloned().collect();

    // or initializing from Vec (Iterator)
    let mut ages: HashMap<&str, i32> = 
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


#[macro_use] extern crate maplit;

fn maplit_crate() {
    let map = hashmap!{
        "daffy" => 80,
        "bugs" => 79,
        "taz" => 63,
    };
    println!("{:#?}", map);
}



fn read_file() {
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    use std::path::Path;

    let fp = File::open(Path::new("/tmp/song.txt")).unwrap();
    let file = BufReader::new(&fp);
    for line in file.lines() {
        println!("{}", line.unwrap());
    } 

}


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


#[derive(Debug)]
struct Actor {
    name: String,
    age: i32
}


fn main() {
    println!("Hello, world!");
    types_and_declarations();
    add(3, 3);
    list_slice();
    dict_map();
    maplit_crate();
    read_file();

    let grumpy = Cat::new("Grumpy");
    grumpy.greet("Garfield");


    let daffy = Actor {name: "Daffy".into(), age: 80};
    println!("{:#?}", daffy);
}
