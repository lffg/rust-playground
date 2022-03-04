macro_rules! cmp {
    ($type:ident) => {
        let start = std::time::Instant::now();
        let mut curr = $type::MIN;
        loop {
            if curr == $type::MAX {
                break;
            }
            curr += 1;
        }
        let elapsed = start.elapsed().as_micros();
        println!("{} finished in {elapsed} microseconds", stringify!($type));
    };
}

fn main() {
    cmp!(u32);
    cmp!(i32);
}
