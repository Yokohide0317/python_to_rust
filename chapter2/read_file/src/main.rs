use std::env; // コマンドライン引数用
use std::fs; // ファイルの読み込み用

fn main() {
    // 引数をベクターとして
    let args: Vec<String> = env::args().collect();
    // ファイル名の指定があるかどうか調べる
    if args.len() < 2 {
        println!("入力ファイルを指定してください.");
        return;
    }
    let filename = &args[1];
    // ファイルを読み込んで表示
    let text = fs::read_to_string(filename).unwrap();
    println!("{}", text);
}
