fn main() {
    let s = String::from("asdads");
    take_owner(s);
    let mut s = String::from("tt");
    s = take_owner_and_give(s);
    println!("{s}");
    
}

//when using primitives, the value is copied
//when using data structures, the ptr is copied and the original variable is dropped unless .copy() is used

fn take_owner(s: String) {
    println!("{s}");
}

fn take_owner_and_give(s: String) -> String {
    println!("{s}");
    s
}