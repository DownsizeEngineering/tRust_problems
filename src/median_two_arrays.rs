fn average(a: i32, b: i32) -> f64 {
    return ((a + b) as f64) / 2.0;
}

fn test_median(ar1: Vec<i32>, ar2: Vec<i32>) -> f64 {
    let mut concat = [&ar1[..], &ar2[..]].concat();
    concat.sort();

    if concat.len() % 2 == 1 {
        return concat[concat.len() / 2] as f64;
    }
    else {
        return average(concat[concat.len() / 2], concat[concat.len() / 2 - 1]);
    }
}


pub fn run() {
    let a = vec![1,3,5,7,9];
    let b = vec![2,4,6,8];

    println!("{}", test_median(a, b));
}