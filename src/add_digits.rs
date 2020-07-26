pub fn run() -> i32 {
    pub fn add_digits(num: i32) -> i32 {
        num - 9 * ((num -1 ) / 9)
    }
    add_digits(5432)
}