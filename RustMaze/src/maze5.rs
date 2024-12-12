use std::cell::RefCell;
use std::sync::Arc;

#[derive(PartialEq, Debug)]
enum Exploration {
    Explored,
    PartiallyExplored,
    UnExplored,
}

type MazeRef<'a> = Arc<Maze<'a>>;

enum Maze<'a> {
    Branch(&'a str, MazeRef<'a>, MazeRef<'a>, RefCell<Exploration>),
    Leaf(&'a str),
}

impl<'a> Maze<'a> {
    fn explore(&self, node: MazeRef<'a>, work: &mut Vec<MazeRef<'a>>, trace: &mut Vec<&'a str>) {
        if let Maze::Branch(label, left, right, status) = self {
            let mut current_status = status.borrow_mut();
            if *current_status == Exploration::UnExplored {
                *current_status = Exploration::PartiallyExplored;
                work.push(node.clone());
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
    let branch3 = Arc::new(Branch("3", Arc::clone(&leaf4), Arc::clone(&leaf5), RefCell::new(UnExplored)));
    let branch1 = Arc::new(Branch("1", Arc::clone(&leaf2), Arc::clone(&branch3), RefCell::new(UnExplored)));
    let branch7 = Arc::new(Branch("7", Arc::clone(&leaf5), Arc::clone(&leaf8), RefCell::new(UnExplored)));
    let branch6 = Arc::new(Branch("6", Arc::clone(&branch3), Arc::clone(&branch7), RefCell::new(UnExplored)));
    Arc::new(Branch("0", Arc::clone(&branch1), Arc::clone(&branch6), RefCell::new(UnExplored)))
}

pub fn main() {
    let maze = maze();
    let mut work = vec![maze.clone()];
    let mut trace = vec![];

    while work.len() != 0 {
        let node = work.pop().expect("work stack should not be empty");
        node.explore(node.clone(), &mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
}
