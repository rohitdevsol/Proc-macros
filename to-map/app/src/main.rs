use macros::{ FromMap, ToMap };
use std::collections::HashMap;
use traits::{ ToMap, FromMap };

#[derive(Debug, ToMap, FromMap)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let person = Person {
        name: String::from("bri"),
        age: 28,
    };
    // println!("{:?}", person);

    // let mut person_hashmap = HashMap::new();
    // person_hashmap.insert("name", person.name);
    // person_hashmap.insert("age", person.age.to_string());

    let person_hashmap = person.to_map(); // after building my macro

    println!("{:?}", person_hashmap);

    let from_map_struct = Person::from_map(person_hashmap);
    println!("{:?}", from_map_struct);
}

// what do we want ?
// person.to_map();
// {"name": "bri", "age": "34"}
