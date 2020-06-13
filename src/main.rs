// mod min_edits;
mod modulo_buddies;

fn main() {
    // min_edits::run();
    println!("{:?}", modulo_buddies::run(vec![1, 2, 3, 4, 6, 9, 12, 18, 36]));
}
