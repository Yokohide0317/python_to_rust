fn main() {
    let s = String::from("吾輩は猫である");
    let ss = &s[0..s.len()];
    println!("{}", ss);
}
