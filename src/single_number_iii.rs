pub fn run () -> Vec<i32> {
    let nums = vec![1,1,2,2,3,3,4,4,5,6];

    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mask = nums.iter().fold(0, |acc, val| acc ^ val);
        let lowbit = mask & -mask;

        let ans = nums.iter().fold((0,0), |acc, val| {
            if val & lowbit == 0 { return (acc.0 ^ val, acc.1); }
            return (acc.0, acc.1 ^ val);
        });
        vec![ans.0, ans.1]
    }

    single_number(nums)
}