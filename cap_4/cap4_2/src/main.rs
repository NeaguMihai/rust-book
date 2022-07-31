fn main() {
    let mut s = String::from("T");
    let ss = &mut s;
    modif_ref(ss);
    println!("{}", s);
}

fn modif_ref(s: &mut String) {
    s.push_str("ch");
}