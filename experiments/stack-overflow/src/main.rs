fn rec(val: u64) {
    let addr = &val;
    println!("{addr:p} (is {val})");
    return rec(val + 1);
}

fn main() {
    rec(0);
}
