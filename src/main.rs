// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!");
//     println!("{}", s);
// }

fn main() {
    let s = String::from("hello");

    // let len = calc_len(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    let word = first_word(&s);

    s.clear();
    println!("The first word is: {}", word);
}

fn calc_len(s: &String) -> usize {
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..] // If no space is found, return the entire string
}
