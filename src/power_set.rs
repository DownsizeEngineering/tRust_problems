pub fn run () -> Vec<Vec<i32>> {

    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let len = nums.len();
        let mut output: Vec<Vec<i32>> = vec![Vec::new()];

        let subsets = recursive_callback(&(Vec::new(), 0), &nums);
        for set in subsets { output.push(set); }

        fn recursive_callback(
            base: &(Vec<i32>, usize), 
            nums: &Vec<i32>) 
        -> Vec<Vec<i32>> {
            let (base, z) = base;
            let len = nums.len();
            let mut sets = Vec::new();

            for i in *z..len {
                let mut new_base = base.clone();
                
                new_base.push(nums[i]);
                sets.push(new_base.clone());
                let subsets = recursive_callback(&(new_base, i+1), nums);
                for set in subsets { sets.push(set); }
            }
            sets
        }

        output
    }
    subsets(vec![1,2,3])
}