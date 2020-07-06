pub fn run() -> Vec<i32> {
    plus_one(vec![9,0,9])
}

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut output = digits.to_vec();
    let len = digits.len();

    for i in 1..=len {
        let i = len - i;
        if output[i] == 9 {
            output[i] = 0;
            if i == 0 {
                output.insert(0, 1);
            }
        }
        else {
            output[i] += 1;
            return output;
        }
    }
    output
}