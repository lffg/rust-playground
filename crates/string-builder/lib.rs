pub struct StringBuilder {
    chars: Vec<char>,
}

impl StringBuilder {
    pub fn new() -> Self {
        StringBuilder { chars: Vec::new() }
    }

    pub fn append(mut self, char: char) -> Self {
        self.chars.push(char);
        self
    }

    pub fn build(self) -> String {
        self.chars.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let str = StringBuilder::new().append('a').append('b').build();
        assert_eq!(str, "ab");
    }
}
