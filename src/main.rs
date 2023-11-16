#[derive(Debug)]
enum NumOrStr {
    Num(i32),
    Str(String),
    Hoge,
}

fn main() {
    let x = NumOrStr::Num(10);
    let y = NumOrStr::Str("waowao".to_string());
    let z = NumOrStr::Hoge;

    println!("x: {:?}, y: {:?}, z: {:?}", x, y, z);
}
