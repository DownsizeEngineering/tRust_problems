pub fn run(n: u8, mut k: u64) -> String {
/*    if n > 20 {
        panic!("Cannot store factorials 21! or greater in a u64");
    }*/
    if n > 9 || n ==0 {panic!("n must be between 1 and 9");}
    let mut p:u64 = 1;

    for x in 1..n {
        p *= x as u64;
    }

    if k > p * n as u64|| k == 0 {
        panic!("permutation outside of the range of p!");
    }

    let mut nums: Vec<char> = Vec::new();
    for x in 1..=n {
        nums.push(std::char::from_digit(x as u32, 10).unwrap());
    }
    let mut output = String::from("");

    for v in 1..=n {
        let i = if k % p == 0 { k / p - 1} else {k / p};
        output.push(nums.remove(i as usize));
        if v == n {break;}
        k -= i * p;
        p /= n as u64 - v as u64;
    }

    output
}