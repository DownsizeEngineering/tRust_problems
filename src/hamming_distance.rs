pub fn run(a: i32, b: i32) -> i32 {
    assert_eq!(hamming_distance(a,b), string_hamming(a, b));
    hamming_distance(a,b)
}

pub fn hamming_distance(x: i32, y: i32) -> i32 {
   let mut z = x ^ y;
   let mut count = 0;
   while (z != 0) {
       z &= z - 1;
       count += 1;
   }
   count
}

pub fn string_hamming(x: i32, y: i32) -> i32 {
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