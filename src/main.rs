fn chan(s: &mut String) {
   s.push_str("-chan");
}

fn main() {
    let mut x = "neko".to_string();
    chan(&mut x);
    println!("{}", x);
}
