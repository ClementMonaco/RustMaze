use std::cell::RefCell;

#[derive(PartialEq, Debug)]
enum Exploration {
    Explored,
    UnExplored,
}

enum Maze<'a> {
    Branch(&'a str, &'a Maze<'a>, &'a Maze<'a>, RefCell<Exploration>),
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

pub fn main() {
    use Maze::*;
    use Exploration::*;

    let leaf2 = Leaf("2");
    let leaf4 = Leaf("4");
    let leaf5 = Leaf("5");
    let leaf8 = Leaf("8");
    let branch3 = Branch("3", &leaf4, &leaf5, RefCell::new(UnExplored));
    let branch1 = Branch("1", &leaf2, &branch3, RefCell::new(UnExplored));
    let branch7 = Branch("7", &leaf5, &leaf8, RefCell::new(UnExplored));
    let branch6 = Branch("6", &branch3, &branch7, RefCell::new(UnExplored));
    let branch0 = Branch("0", &branch1, &branch6, RefCell::new(UnExplored));

    let mut trace = Vec::new();
    branch0.explore(&mut trace);
    println!("Explored nodes: {:?}", trace);
}
