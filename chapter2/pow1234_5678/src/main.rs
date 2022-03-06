use num_bigint::BigInt;

fn main() {
    // BigIntのオブジェクトでオーバーフローしない様に値を設定
    let v = BigInt::from(1234);
    // 累乗の計算
    println!("{}", v.pow(5678));
}
