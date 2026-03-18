use macros::Diff;
use traits::Diff;

#[derive(Diff, Debug)]
struct Person {
    name: String,
    age: i32,
}

fn main() {
    let a = Person {
        name: String::from("Lord subh"),
        age: 21,
    };
    let b = Person {
        name: String::from("beautiful bri"),
        age: 28,
    };

    println!("{:?}", a);

    let diff = a.diff(&b);
    println!("{:?}", diff);
}

// what do we want ??
// let diff = a.diff(b);
// ["age changed form : {} to {}", "name changed form : {} to {}"]
