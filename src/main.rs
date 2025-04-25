// fn main() {
//     let mut s = String::from("hello");

//     s.push_str(", world!");
//     println!("{}", s);
// }

fn main() {
    let s1 = String::from("hello");

    let len = calc_len(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calc_len(s: &String) -> usize {
    s.len()
}
