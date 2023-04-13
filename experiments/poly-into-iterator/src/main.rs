use std::fmt::Display;

pub fn print_all<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Display,
{
    for el in iter {
        println!("{el}");
    }
}

fn main() {
    let els = vec![1, 2, 3];
    print_all(&els);
    print_all(els);
}
