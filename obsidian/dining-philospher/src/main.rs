use dining_philospher::{Fork, Philospher};

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;


fn main() {
    println!("Dining Philosophers Problem:  15 Philosophers, 4 Forks...Yikes!!");


    let forks = (0..4)
                                        .map(|id| {
                                            Arc::new(
                                                Fork {
                                                    id, 
                                                    mutex: Mutex::new(()),
                                                }
                                            )
                                        }).collect::<Vec<_>>();
    

    let philosophers = vec![
                                            ("Jürgen Habermas", 0, 1),
                                            ("Friedrich Engels", 1, 2),
                                            ("Karl Marx", 2, 3),
                                            ("Thomas Piketty", 3, 0),
                                            ("Michel Foucault", 0, 1),
                                            ("Socrates", 1, 2),
                                            ("Plato", 2, 3),
                                            ("Aristotle", 3, 0),
                                            ("Pythagoras", 0, 1),
                                            ("Heraclitus", 1, 2),
                                            ("Democritus", 2, 3),
                                            ("Diogenes", 3, 0),
                                            ("Epicurus", 0, 1),
                                            ("Zeno of Citium", 1, 2),
                                            ("Thales of Miletus", 2, 3),
                                        ]
                                        .into_iter()
                                        .enumerate()
                                        .map(|(id, (name, left_fork_id, right_fork_id))| {
                                            Philospher::new(
                                                id as usize,
                                                name,
                                                Arc::clone(&forks[left_fork_id]),
                                                Arc::clone(&forks[right_fork_id]),
                                            )
                                        }).collect:: <Vec<_>>();
 
    
    let start = Instant::now();

    let handles = philosophers
                                        .into_iter()
                                        .map(|philosopher| {
                                            thread::spawn(move || {
                                                philosopher.eat();
                                            })
                                        }).collect:: <Vec<_>>();

    for handle in handles {
        match handle.join() {
            Ok(_) => {},
            Err(_) => print!("Could not join"),
        }
    }


    println!("Total time: {:?}", start.elapsed());
}
