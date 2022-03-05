fn main() {
    // 整数なのか、少数なのかはっきりしないといけない
    let moon = 384400.0;
    let car = 80.0;
    let btrain = 300.0;
    println!("car: {}", moon / car / 24.0);
    println!("btrain: {}", moon / btrain / 24.0);
}
