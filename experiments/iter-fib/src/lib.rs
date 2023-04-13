pub fn fib() -> impl Iterator<Item = u64> {
    let mut a = 0;
    let mut b = 1;

    std::iter::from_fn(move || {
        let out = a;
        a = b;
        b = out + a;
        Some(out)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let out: Vec<_> = fib().take(10).collect();
        assert_eq!(out, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }
}
