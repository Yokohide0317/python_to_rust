fn main() {
    let a_vec: Vec<u32> = vec![100, 200, 300];
    for a in a_vec {
        println!("{}", a);
    }

    let s_vec: Vec<&str> = vec!["犬", "猫", "鶏"];
    for s in s_vec {
        println!("{}", s)
    }
}
