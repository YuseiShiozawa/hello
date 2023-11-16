fn chan(s: &String) -> String {
   s.clone() + "-chan"
}

fn main() {
    let x = "neko".to_string();
    let y = chan(&x);
    println!("{}", y);
    println!("{}", x);
}
