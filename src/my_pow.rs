pub fn run() -> f64 {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 1.0 { return x;}
        if x == -1.0 {
            if n % 2 == 0 { return 1.0; }
            return -1.0; 
        }
        if n < -2147483647 { return 0.0; }
        if n == 0 { return 1.0; }
        let (mut x, mut n) = (x, n);
        if n < 0 {
            x = 1.0 / x;
            n = -1 * n;
        }
        if n == 1 { return x; }

        let mut output = x;
        for _ in 1..n {
            output *= x;
            if output.is_infinite() {
                if x < 0.0 && n % 2 == 1 { return std::f64::NEG_INFINITY; }
                return std::f64::INFINITY;
            }
            if output == 0.0 { return 0.0; }
        }
        output
    }
    my_pow(10000000000.00000, -2147483648)
}