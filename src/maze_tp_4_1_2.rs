use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::channel;
use std::thread::yield_now;

#[derive(PartialEq, Debug)]
enum Exploration {
    Explored,
    PartiallyExplored,
    UnExplored,
}

type MazeRef<'a> = Arc<Maze<'a>>;

enum Maze<'a> {
    Branch(&'a str, MazeRef<'a>, MazeRef<'a>, Mutex<Exploration>),
    Leaf(&'a str),
}

impl<'a> Maze<'a> {
    fn explore(&self, node: MazeRef<'a>, work: &Arc<Mutex<Vec<MazeRef<'a>>>>, trace: &mut Vec<&'a str>) {
        if let Maze::Branch(label, left, right, status) = self {
            let mut current_status = status.lock().unwrap();
            if *current_status == Exploration::UnExplored {
                *current_status = Exploration::PartiallyExplored;
                work.lock().unwrap().push(node.clone());
                left.explore(left.clone(), work, trace);
            } else if *current_status == Exploration::PartiallyExplored {
                *current_status = Exploration::Explored;
                right.explore(right.clone(), work, trace);
            } else if *current_status == Exploration::Explored {
                trace.push(*label);
            }
        } else if let Maze::Leaf(label) = self {
            trace.push(*label);
        }
    }
}

fn maze<'a>() -> MazeRef<'a> {
    use Maze::*;
    use Exploration::*;

    let leaf2 = Arc::new(Leaf("2"));
    let leaf4 = Arc::new(Leaf("4"));
    let leaf5 = Arc::new(Leaf("5"));
    let leaf8 = Arc::new(Leaf("8"));
    let branch3 = Arc::new(Branch("3", Arc::clone(&leaf4), Arc::clone(&leaf5), Mutex::new(UnExplored)));
    let branch1 = Arc::new(Branch("1", Arc::clone(&leaf2), Arc::clone(&branch3), Mutex::new(UnExplored)));
    let branch7 = Arc::new(Branch("7", Arc::clone(&leaf5), Arc::clone(&leaf8), Mutex::new(UnExplored)));
    let branch6 = Arc::new(Branch("6", Arc::clone(&branch3), Arc::clone(&branch7), Mutex::new(UnExplored)));
    Arc::new(Branch("0", Arc::clone(&branch1), Arc::clone(&branch6), Mutex::new(UnExplored)))
}

pub fn main() {
    let maze = maze();
    let work = Arc::new(Mutex::new(vec![maze.clone()]));
    let (sender, receiver) = channel();
    let num_threads = 4;
    let mut handles = vec![];

    for i in 0..num_threads {
        let work_clone = Arc::clone(&work);
        let sender_clone = sender.clone();
        let handle = thread::spawn(move || {
            let mut trace = vec![];
            loop {
                let node = {
                    let mut work_guard = work_clone.lock().unwrap();
                    work_guard.pop()
                };
                match node {
                    Some(node) => {
                        node.explore(node.clone(), &work_clone, &mut trace);
                        println!("Worker {} explored nodes: {:?}", i, trace);
                        yield_now();
                    }
                    None => break,
                }
            }
            sender_clone.send(trace).unwrap();
        });
        handles.push(handle);
    }

    drop(sender);

    let mut final_trace = vec![];
    for trace in receiver {
        final_trace.extend(trace);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final trace: {:?}", final_trace);
}
