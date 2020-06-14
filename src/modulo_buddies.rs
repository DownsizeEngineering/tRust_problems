/*******************************************************************************
    Modulo Buddies:

    Number Y is number X's modulo buddy if X % Y == 0

    Given an unsorted set of unique positive integers, find the longest subset
    such that each smaller number is a modulo buddy of each larger number
*******************************************************************************/

pub fn run (nums: Vec<u32>) -> Vec<u32> {

    let mut vec = nums.to_vec();
    vec.sort();
    // backtrack(vec)   
    dp(vec)
}

fn backtrack(nums: Vec<u32>) -> Vec<u32> {
    if nums.len() == 0 {return nums}
    let mut output = Vec::new();

    fn recursive_call (
        mut current: Vec<u32>,
        next: usize,
        output: &mut Vec<u32>,
        nums: &Vec<u32>
        ) {
        let val = nums[next];
        for member in current.iter() {
            if val % member != 0 {return;}
        }
        current.push(val);
        let next = next + 1;
        for i in next..nums.len() {
            recursive_call(current.to_vec(), i, output, nums);
        }
        if current.len() >= output.len() {*output = current;}
    };
    for i in 0..nums.len() - 1 { //check only up the penultimate item
        recursive_call(Vec::new(), i, &mut output, &nums);
    }

    output
}

fn dp(nums: Vec<u32>) -> Vec<u32> {
    use std::collections::HashMap;
    type ValCount = (u32, usize);

    let mut output = Vec::new();
    let mut paths: HashMap<u32, ValCount> = HashMap::new();
    let mut max: ValCount = (0, 0);
    
    for i in 0..nums.len() {
        let num = nums[i];
        let mut path: ValCount = (num, 0);
        for j in 1..=i {
            let cur = nums[i - j];
            let curl = paths.get(&cur).unwrap().1;
            if num % cur == 0 && curl >= path.1 {
                path = (cur, curl + 1);
            }
        }
        if path.1 >= max.1 {max = (num, path.1);}
        paths.insert(num, path);
    }
    
    let mut num = max.0;
    loop {
        output.push(num);
        let path = paths.get(&num).unwrap();
        if path.1 == 0 {break;}
        num = path.0;
    }

    output.into_iter().rev().collect()
}