use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;


pub struct Fork {
    pub id: usize,
    pub mutex: Mutex<()>,
}


pub struct Philospher {
    id: usize,
    name: String,
    left_fork: Arc<Fork>,
    right_fork: Arc<Fork>,
}


impl Philospher {
    pub fn new(id: usize, name: &str, left_fork: Arc<Fork>, right_fork: Arc<Fork>) -> Philospher {
        Philospher {
            id,
            name: String::from(name),
            left_fork,
            right_fork,
        }
    }

    pub fn eat(&self) {
        let (first_fork, second_fork) = if (self.id % 2) == 0 {
            (&self.left_fork, &self.right_fork)
        } else {
            (&self.right_fork, &self.left_fork)
        };

        // first_gaurd
        if let Ok(_) = first_fork.mutex.lock() {
            println!("{} picked up fork {}.", self.name, first_fork.id);
        } else {
            print!("First Gaurd Failed for {} of Fork ID: {}", self.name, first_fork.id);
        }

        // second_gaurd
        match second_fork.mutex.lock() {
            Ok(_) => println!("{} picked up fork {}.", self.name, first_fork.id),
            Err(_) => print!("First Gaurd Failed for {} of Fork ID: {}", self.name, second_fork.id),
        }


        println!("{} is eating!", self.name);

        thread::sleep(Duration::from_secs(1));

        println!("{} finished eating!", self.name);
        
        println!("{} put down the fork {}", self.name, first_fork.id);
        println!("{} put down the fork {}", self.name, second_fork.id);
    }
}
