use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug)]
enum Exploration {
    Explored,
    UnExplored,
}

type MazeRef<'a> = Rc<Maze<'a>>;

enum Maze<'a> {
    Branch(&'a str, MazeRef<'a>, MazeRef<'a>, RefCell<Exploration>),
    Leaf(&'a str),
}

impl<'a> Maze<'a> {
    fn explore(&self, trace: &mut Vec<&'a str>) {
        if let Maze::Branch(label, left, right, status) = self {
            let mut current_status = status.borrow_mut();
            if *current_status == Exploration::UnExplored {
                *current_status = Exploration::Explored;
                trace.push(*label);
                left.explore(trace);
                right.explore(trace);
            } else {
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

    let leaf2 = Rc::new(Leaf("2"));
    let leaf4 = Rc::new(Leaf("4"));
    let leaf5 = Rc::new(Leaf("5"));
    let leaf8 = Rc::new(Leaf("8"));
    let branch3 = Rc::new(Branch("3", Rc::clone(&leaf4), Rc::clone(&leaf5), RefCell::new(UnExplored)));
    let branch1 = Rc::new(Branch("1", Rc::clone(&leaf2), Rc::clone(&branch3), RefCell::new(UnExplored)));
    let branch7 = Rc::new(Branch("7", Rc::clone(&leaf5), Rc::clone(&leaf8), RefCell::new(UnExplored)));
    let branch6 = Rc::new(Branch("6", Rc::clone(&branch3), Rc::clone(&branch7), RefCell::new(UnExplored)));
    Rc::new(Branch("0", Rc::clone(&branch1), Rc::clone(&branch6), RefCell::new(UnExplored)))
}

pub fn main() {
    let labyrinth = maze();

    let mut trace = Vec::new();
    labyrinth.explore(&mut trace);
    println!("Explored nodes: {:?}", trace);
}
