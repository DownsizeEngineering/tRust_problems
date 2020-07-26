pub fn run() -> i32 {
    pub fn add_digits(num: i32) -> i32 {
        let mut num = num;
        let mut output = 0;
        while num > 0 {
            let digit = num % 10;
            num -= digit;
            output += digit;
            if output > 9 { output -= 9; }
            num /= 10;
        }
        output
    }
    add_digits(5432)
}