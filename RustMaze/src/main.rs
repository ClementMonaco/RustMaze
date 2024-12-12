mod maze3;
mod maze4;
mod maze5;

fn main() {
    println!("Hello, world!");
    println!("Maze 3 avec les références");
    maze3::main();
    println!("Maze 4 avec le pointeur intelligent Rc");
    maze4::main();
    println!("Maze 5 avec la concurrence");
    maze5::main();
}
