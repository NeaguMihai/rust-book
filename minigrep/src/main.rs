// use std::{env, fs};

// fn main() {
//     let args: Vec<String> = env::args().collect();
//     let config = Config::build(&args).expect("Error parsing arguments");

//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);

//     let content = fs::read_to_string(config.file_path).expect("Something went wrong reading the file");

//     println!("With text: {content}");
// }

// struct Config {
//     query: String,
//     file_path: String,
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> {
//         if args.len() < 3 {
//             return Err("not enough arguments");
//         }

//         let query = args[1].clone();
//         let file_path = args[2].clone();

//         Ok(Config { query, file_path })
//     }
// }

fn main() {

}

struct Node<T> {
    value: Box<T>,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        let mut node = Node {
            value: Box::new(value),
            next: None
        };
        node
    }

    fn next(&self) -> Option<&Box<Node<T>>> {
        self.next.as_ref()
    }

}