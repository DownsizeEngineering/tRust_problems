pub fn run(a: i32, b: i32) -> i32 {
    hamming_distance(a,b)
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
        let mut x: String = format!("{:b}", x); 
        let mut y: String = format!("{:b}", y); 

        let mut count = 0;

        loop {
            let x = x.pop();
            let y = y.pop();
            if x == None && y == None { break; }
       
            let x = match x {
                Some(z) => z,
                None => '0',
            };

            let y = match y {
                Some(z) => z,
                None => '0',
            };

            if x != y { count += 1; }
        }

        count
    }