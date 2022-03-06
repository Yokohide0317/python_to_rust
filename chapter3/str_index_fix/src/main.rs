fn main() {
    let s = "Hello. こんにちは";
    let ch = s.chars().nth(4).unwrap();
    println!("{}", ch);

    let ch = s.chars().nth(6).unwrap();
    println!("{}", ch);
}
