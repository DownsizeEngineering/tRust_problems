pub fn run() -> Vec<i32> {
    let elems = vec![1,1,1,2,2,3,3,3,3,3,3,3];
    let k = 2;
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut output = Vec::new();
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            let count = counts.get_mut(&num);
            if let Some(count) = count { *count += 1; }
            else { counts.insert(num, 1); }
        }

        let mut counts: Vec<(&i32, &i32)> = counts.iter().collect();

        counts.sort_by_key(|k| k.1);

        for i in 0..k as usize {
            output.push(*(counts.pop().unwrap().0));
        }
        output
    }
    top_k_frequent(elems, k)
}