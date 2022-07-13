#![allow(dead_code)]

fn read<T>() -> T
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    T::from_str(buf.trim()).unwrap()
}

fn read_many<T>() -> Vec<T>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();

    buf.split(' ')
        .map(|s| T::from_str(s.trim()).unwrap())
        .collect()
}
