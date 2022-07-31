fn main() {
    let s = String::from("AAA hhh");
    let bytes = s.as_bytes();

    #[allow(unused_variables)]
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            break;
        }
    }

    println!("{}", s.len());
    slice();
}


fn slice() {
    let s = String::from("asdfg");
    #[allow(unused_variables)]
    let s1 = &s[0..6];

    let a = [7, 8, 9, 6, 4];

    #[allow(unused_variables)]
    let a1 = &a[0..2];
}