fn main() {
    for x in 1..10 {
        for y in 1..10 {
            print!("{:4},", x * y);
        }
        print!("\n")
    }
}
