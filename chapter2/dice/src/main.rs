use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let dice = rng.gen_range(1..7);
        println!("{}", dice);
    }
}
