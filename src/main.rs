// mod min_edits;
// mod modulo_buddies;
// mod dijkstra_preflight;
mod which_permutation;

fn main() {
    // min_edits::run();
    // println!("{:?}", modulo_buddies::run(vec![2, 3, 5, 7, 11, 13, 17, 19]));
    /*dijkstra_preflight::run( vec![
        (0, 1, 1), 
        (0, 4, 500), 
        (1, 2, 1), 
        (2, 3, 1), 
        (3, 4, 1), 
        (4, 5, 1)], 

        (0,5,4) );*/
        println!("{}", which_permutation::run(9,500));
}
