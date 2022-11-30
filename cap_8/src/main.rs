use std::fmt::format;
use std::collections::HashMap;


fn main() {
    vector();
    strings();
    hash_maps();
}

fn vector() {
    let mut v = vec!(1, 2, 3);
    //insert
    v.push(4);

    //READ
    //read by index (might be unsafe if the index is out of bounds)
    println!("The third element of the vector is {}", &v[2]);

    //read with get() (returns an Option<&T>) (safe) but it's way more verbose
    match v.get(2) {
        Some(third) => println!("The third element of the vector is {}", third),
        None => println!("There is no third element")
    }

    //Iteration

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn strings() {
    let a = "asdsad";
    let b = a.to_string() + "asd";
    let c = format!("{}asdasd", a);
    println!("{}   {}", b, c);

    let mut s = String::from("foo");
    let a = String::from("bar");
    s.push_str(a.as_str());
    println!("{}", a);
    println!("{}", s);
}
fn hash_maps() {

    let text = "hello world wonderful world";
    let mut map:HashMap<&str, u128> = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}