fn main() {
    let mut v = vec![1, 2, 3];
    v.insert(0, 10000);
    v.pop();
    println!("{:?}", &v);
}
