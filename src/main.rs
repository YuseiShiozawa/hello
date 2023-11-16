#[derive(Debug)]
enum NumOrStr {
    Num(i32),
    Str(String),
    Hoge,
}

fn main() {
    let mut v = vec![NumOrStr::Num(10)];
    v.push( NumOrStr::Str("waowao".to_string()) );
    v.push( NumOrStr::Hoge );

    println!("{:?}", &v);

    for e in v {
        match e {
            NumOrStr::Num(n) => println!("数字だよ {}", n),
            NumOrStr::Str(s) => println!("文字列だよ {}", s),
            NumOrStr::Hoge   => println!("HOGE!!!!!!!!!!!!!!!"),

        }
    }

}
