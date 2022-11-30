use application_structure::frontend;

fn main() {
    frontend::get_users().iter().for_each(|user| println!("{}", user));
}
