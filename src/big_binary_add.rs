pub fn run() -> String {
    let a = String::from("110000001");
    let b = String::from("1");

    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a;
        let mut b = b;

        let mut output = String::new();
        let mut carry = false;
        while !(a.is_empty() && b.is_empty()) {
            let a = match a.pop() {
                Some(x) => x,
                None => '0'
            };
            let b = match b.pop() {
                Some(x) => x,
                None => '0'
            };

            let c = a.to_digit(2).unwrap() 
                  + b.to_digit(2).unwrap() 
                  + carry as u32;
            carry = false;
            if c > 1 { carry = true; }
            output.push(std::char::from_digit(c % 2, 2).unwrap());

        }
        if carry {output.push('1');}
        output.chars().rev().collect::<String>()
    }

    add_binary(a, b)
}