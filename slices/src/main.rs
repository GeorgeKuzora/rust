fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("Hello, {word}!");
    s.clear();
    println!("Hello, {s}!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &elem) in bytes.iter().enumerate() {
        if elem == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
