pub fn run () -> String {

    pub fn reverse_words(s: String) -> String {
        s.split_ascii_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }

    reverse_words(String::from("a good    example"))
}