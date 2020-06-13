/*******************************************************************************
    Modulo Buddies:

    Number Y is number X's modulo buddy if X % Y == 0

    Given an unsorted set of unique positive integers, find the longest subset
    such that each smaller number is a modulo buddy of each larger number
*******************************************************************************/

pub fn run (nums: Vec<u32>) -> Vec<u32> {

    let mut vec = nums.to_vec();
    vec.sort();
    backtrack(vec)   
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

    recursive_call(Vec::new(), 0, &mut output, &nums);

    output
}