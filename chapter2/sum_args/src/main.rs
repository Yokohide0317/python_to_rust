fn main() {
    // コマンドライン引数を得る。
    let args = std::env::args();
    let mut total = 0.0;
    // コマンドライン引数を順に加算。
    for (i, s) in args.enumerate() {
        // 0番目はコマンド
        if i == 0 {
            continue;
        }
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        };
        total += num;
    }
    println!("{}", total);
}
