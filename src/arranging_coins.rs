pub fn run() -> i32 {
    arrange_coins(-20)
}

pub fn arrange_coins(n: i32) -> i32 {
        let mut row = 0;
        let mut coins = n;

        loop {
            let new_row = row + 1;
            coins -= new_row;
            if coins >= 0 { row = new_row; }
            if coins <= 0 { break; }
        }
        row
    }


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_values() {
        assert_eq!(arrange_coins(5), 2);
        assert_eq!(arrange_coins(0), 0);
        assert_eq!(arrange_coins(8), 3);
        assert_eq!(arrange_coins(-20), 0);

    }
}