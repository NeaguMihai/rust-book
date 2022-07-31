fn main() {
    println!("Hello, world!");
    if_stmt();
    if_thernary();
    for_in();
    for_in_range();
}

fn if_stmt() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_thernary() {
    let a = 7;

    let res = if a < 9 { 8 } else { 3 };

    println!("{res}");
}

//there is loop -> can stop only with break
//there is while -> classic


fn for_in() {
    let a: [i32; 4] = [4, 7, 9, 3];
    for el in a {
        println!("{el}");
    }
}

fn for_in_range() {
    let a: i32 = 10;
    for el in 0..a {
        println!("{el}");
    }
}