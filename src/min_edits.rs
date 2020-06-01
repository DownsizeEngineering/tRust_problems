pub fn run() {
    println!("Hello from min_edits");
    println!("{}", min_edits("banana", "phone"));
    
}

fn min3 (a: usize, b:usize, c:usize) -> usize {
    std::cmp::min(std::cmp::min(a,b), c)
}

fn min_edits(a: &str, b: &str) -> u64 {
    println!("{} {}", a, b);
    let mut dp: Vec<Vec<usize>> = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        for j in 0..=b.len() {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if a.get(i-1..i).unwrap() == b.get(j-1..j).unwrap() {
                dp[i][j] = dp[i-1][j-1];
            } else {
                dp[i][j] = 1 + min3(
                    dp[i][j-1],
                    dp[i-1][j],
                    dp[i-1][j-1]
                );
            }
        }
    }
    dp[a.len()][b.len()] as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn horse_ros() {
        assert_eq!(min_edits("horse", "ros"), 3);
    }
}