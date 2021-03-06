use rand::seq::SliceRandom;

fn main() {
    // 1から75までの数値を配列に代入
    let mut nums = [0; 75];
    for i in 1..76 {
        nums[i - 1] = i;
    }

    let mut rng = rand::thread_rng();
    nums.shuffle(&mut rng);

    for y in 0..5 {
        for x in 0..5 {
            let i = y * 5 + x;
            if i == 12 {
                //ワイルドカード
                print!("  *,");
            } else {
                print!("{:3},", nums[i]);
            }
        }
        println!("");
    }
}
