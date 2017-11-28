use std::collections::HashMap;


fn types_and_declarations() {
    let age = 80;
    let name = "daffy";
    let weight = 62.3;
    let mut loons = vec!["bugs", "daffy", "taz"];
    loons.push("foo");

    let mut ages = HashMap::new();   // Correct for 2017
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


#[macro_use] extern crate maplit;

fn maplit_crate() {
    let map = hashmap!{
        "daffy" => 80,
        "bugs" => 79,
        "taz" => 63,
    };
    println!("{:#?}", map);
}


fn main() {
    println!("Hello, world!");
    types_and_declarations();
    add(3, 3);
    list_slice();
    dict_map();
    maplit_crate();
}
