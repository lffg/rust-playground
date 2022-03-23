use std::io::{self, BufRead};

fn main() {
    let xs: Vec<i32> = io::stdin()
        .lock()
        .lines()
        .flat_map(IntoIterator::into_iter)
        .flat_map(|str| {
            str.split_ascii_whitespace()
                .flat_map(str::parse)
                // Heap allocation is needed since `SplitWhitespace` (returned by
                // `split_ascii_whitespace` is parameterized over `str`'s lifetime, which cannot
                // be extended beyond the scope of this closure.
                .collect::<Vec<_>>()
                // Here, a new iterator is created by the Vec and moved out of this closure.
                .into_iter()
        })
        .take(10)
        .collect();

    println!("{xs:?}");
}
