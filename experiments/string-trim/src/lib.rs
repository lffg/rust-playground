pub fn ascii_trim(s: &str) -> &str {
    let Some(lo) = s
        .bytes()
        .position(|c| !c.is_ascii_whitespace())
    else {
        return "";
    };
    let hi = s
        .bytes()
        .rev()
        .position(|c| !c.is_ascii_whitespace())
        .map(|i| s.len() - i)
        .unwrap(); // This case is impossible as per the previous `return`.
    &s[lo..hi]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        let cases = &[
            ("abc", "abc"),
            (" abc", "abc"),
            ("  abc", "abc"),
            ("abc ", "abc"),
            ("abc  ", "abc"),
            (" abc ", "abc"),
            ("  abc  ", "abc"),
            ("", ""),
            (" ", ""),
            ("  ", ""),
        ];
        for (input, expected) in cases {
            println!("testing [{input}], must be [{expected}]");
            let out = ascii_trim(input);
            assert_eq!(out, *expected);
        }
    }
}
