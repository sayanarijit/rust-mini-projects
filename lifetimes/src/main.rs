fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn main() {
    let s1 = "abcd";
    let s2 = "xyz";
    println!("Longest: {}", longest(s1, s2));
}
