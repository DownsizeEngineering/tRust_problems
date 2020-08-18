pub fn run() {
    println!("{:?}", fun(4, 2));
}

fn fun(n: u8, k: u8) -> Vec<i32> {
    if n == 0 {return Vec::new();}
    if n == 1 {return vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];}

    let mut output = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];

    for _ in 1..n {
        let mut next = Vec::new();
        for num in &output {
            let digit = num % 10;
            let num = num * 10;
            if k == 0 {
                next.push(num + digit);
            }
            else {
                let down = digit - k as i32;
                let up = digit + k as i32;

                if down >= 0 {
                    next.push(num + down);
                }

                if up < 10 {
                    next.push(num + up);
                }
            }
        }
        output = next;
    }
    output
}