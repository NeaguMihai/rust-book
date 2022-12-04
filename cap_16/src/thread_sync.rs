use std::{sync::{mpsc::{self, Receiver}, Mutex, Arc}, thread, collections::HashMap, error::Error};

pub fn test_message() -> Result<Receiver<String>, Box<dyn std::error::Error>> {
    let (sender, receiver) = mpsc::channel();
    thread::spawn(move|| {
      sender.send(String::from("asd")).unwrap();
  });
  Ok(receiver)
}

pub fn test_di() -> Result<Container, Box<dyn Error>> {
  let mut container = Container::new();
  
  container.add("test", Injectable {});
  Ok(container)
}

pub fn get_instance<'a>(container: &'a Arc<Mutex<Container>>) -> Result<(), Box<dyn Error>> {
  let _container = container.clone();
  let handler = thread::spawn(move || {
    let test = _container.lock().unwrap();
    let val = test.get("test").unwrap();
    val.test();
  });
  let _container2 = container.clone();
  let handler2 = thread::spawn(move || {
    let test = _container2.lock().unwrap();
    let val = test.get("test").unwrap();
    val.test();
  });
  handler.join().unwrap();
  handler2.join().unwrap();
  Ok(())
}

pub struct Container {
    inscances: HashMap<String, Box<Injectable>>
}

struct Injectable {}

impl Injectable {
    fn test(&self) {
        println!("test");
    }
}

impl Container {
    fn new() -> Self {
        let container = Container {
            inscances: HashMap::new()
        };
        container
    }

    fn get(&self, name: &str) -> Option<&Box<Injectable>> {
        let instance = self.inscances.get(name);
        instance
    }

    fn add(&mut self, name: &str, instance: Injectable) {
        self.inscances.insert(String::from(name), Box::new(instance));
    }
}