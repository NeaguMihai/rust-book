use cap_16::route;

#[route("/test", "hjhj")]
pub fn service () {
    println!("test");
}
