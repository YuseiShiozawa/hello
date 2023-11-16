fn chan(s: String) -> String {
   s + "-chan"
}

fn main() {
    let x = "neko".to_string();
    let y = chan(x);
    println!("{}", y);
}
