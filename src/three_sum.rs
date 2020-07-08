pub fn run () -> Vec<Vec<i32>>{
let v = vec![-1, 0, 1, 2, -1, -4];


    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashSet;
        let mut output: Vec<Vec<i32>> = Vec::new();
        let len = nums.len();
        if len < 3 { return output; }
        let mut results = HashSet::new();

        let mut nums = nums.clone();
        nums.sort();

        for i in 0..len - 2 {
            for j in i+1..len - 1 {
                let k = 0 - nums[i] - nums[j];
                if sorted_slice_contains(&nums[j + 1..], k) {
                    results.insert(vec![nums[i], nums[j], k]);
                }
            }
        }

        for triplet in results.iter() {
            output.push(triplet.clone());
        }
        output
    }

    three_sum(v)
}

fn sorted_slice_contains<T: Ord>(slice: &[T], target: T) -> bool {
    if slice.is_empty() {return false;}
    if slice.len() == 1 {return slice[0] == target;}

    let (mut left, mut right) = (0, slice.len() - 1);
    let mut curr: usize;

    return loop {
        curr = (right - left) / 2 + left;
        if slice[curr] == target { break true; }
        if curr == left {break slice[right] == target;}

        if slice[curr] > target { right = curr; } 
        else { left = curr; }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_slice_contains () {
        let v = vec![-4, -3, -2, -1, 0, 1, 2, 3, 4];
        assert!(sorted_slice_contains(&v[..], 4));
        assert!(sorted_slice_contains(&v[..], 3));
        assert!(sorted_slice_contains(&v[..], -4));
        assert!(sorted_slice_contains(&v[..], -3));
        assert!(sorted_slice_contains(&v[0..8], 3));
        assert!(sorted_slice_contains(&v[0..8], 2));
        assert!(sorted_slice_contains(&v[0..8], -4));
        assert!(sorted_slice_contains(&v[0..8], -3));

        assert!(sorted_slice_contains(&[0, 1], 1));
        assert!(sorted_slice_contains(&[0, 1], 0));
        assert!(sorted_slice_contains(&[0], 0));




    }

}

