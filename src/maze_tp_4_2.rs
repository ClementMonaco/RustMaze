use std::sync::{Arc, Mutex};
use std::sync::mpsc;
use std::thread;

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
    Exit(&'a str),
}

impl<'a> Maze<'a> {
    fn explore(&self, node: MazeRef<'a>, work: &Arc<Mutex<Vec<MazeRef<'a>>>>, trace: &mut Vec<&'a str>, found_exit: &Arc<Mutex<bool>>) {
        if *found_exit.lock().unwrap() {
            return;
        }

        if let Maze::Branch(label, left, right, status) = self {
            let mut current_status = status.lock().unwrap();
            if *current_status == Exploration::UnExplored {
                *current_status = Exploration::PartiallyExplored;
                work.lock().unwrap().push(node.clone());
                trace.push(*label);
                left.explore(left.clone(), work, trace, found_exit);
            } else if *current_status == Exploration::PartiallyExplored {
                *current_status = Exploration::Explored;
                trace.push(*label);
                right.explore(right.clone(), work, trace, found_exit);
            } else if *current_status == Exploration::Explored {
                trace.push(*label);
            }
        } else if let Maze::Leaf(label) = self {
            trace.push(*label);
        } else if let Maze::Exit(label) = self {
            trace.push(*label);
            let mut exit_found = found_exit.lock().unwrap();
            *exit_found = true;
        }
    }
}

fn maze<'a>() -> MazeRef<'a> {
    use Maze::*;
    use Exploration::*;

    let leaf2 = Arc::new(Leaf("2"));
    let leaf4 = Arc::new(Leaf("4"));
    let exit5 = Arc::new(Exit("5"));
    let leaf8 = Arc::new(Leaf("8"));
    let branch3 = Arc::new(Branch("3", Arc::clone(&leaf4), Arc::clone(&exit5), Mutex::new(UnExplored)));
    let branch1 = Arc::new(Branch("1", Arc::clone(&leaf2), Arc::clone(&branch3), Mutex::new(UnExplored)));
    let branch7 = Arc::new(Branch("7", Arc::clone(&exit5), Arc::clone(&leaf8), Mutex::new(UnExplored)));
    let branch6 = Arc::new(Branch("6", Arc::clone(&branch3), Arc::clone(&branch7), Mutex::new(UnExplored)));
    Arc::new(Branch("0", Arc::clone(&branch1), Arc::clone(&branch6), Mutex::new(UnExplored)))
}

pub fn main() {
    let maze = maze();
    let work = Arc::new(Mutex::new(vec![maze.clone()]));
    let found_exit = Arc::new(Mutex::new(false));
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    let thread_count = 4;

    for i in 0..thread_count {
        let work = Arc::clone(&work);
        let found_exit = Arc::clone(&found_exit);
        let tx = tx.clone();

        let handle = thread::spawn(move || {
            let mut trace = vec![];
            while let Some(node) = {
                let mut work_guard = work.lock().unwrap();
                if *found_exit.lock().unwrap() || work_guard.is_empty() {
                    None
                } else {
                    Some(work_guard.pop().unwrap())
                }
            } {
                node.explore(node.clone(), &work, &mut trace, &found_exit);
                println!("Worker {} explored nodes: {:?}", i, trace);
                thread::yield_now();
            }
            tx.send((i, trace)).unwrap();
        });
        handles.push(handle);
    }

    drop(tx);

    println!("Final traces :");
    for (i, trace) in rx {
        println!("Worker {} explored nodes: {:?}", i, trace);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
