mod maze3;
mod maze4;
mod maze5;
mod maze_tp_4_1_2;
mod maze_tp_4_2;

fn main() {
    println!("Hello, world!");
    println!("Maze 3 avec les références");
    maze3::main();
    println!("Maze 4 avec le pointeur intelligent Rc");
    maze4::main();
    println!("Maze 5 avec le pointeur intelligent Arc");
    maze5::main();
    println!("Maze TP 4 1.2 avec la concurrence");
    maze_tp_4_1_2::main();
    println!("Maze TP 4 2 avec la sortie du labyrinthe");
    maze_tp_4_2::main();
}
