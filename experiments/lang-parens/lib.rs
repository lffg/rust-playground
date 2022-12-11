pub fn validate(src: &str) -> Result<(), Error> {
    let mut stack = Vec::new();

    for char in src.chars() {
        let tok = Tok::from_char(char)?;
        if tok.opens() {
            stack.push(tok);
            continue;
        }
        let Some(expected) = stack.pop().map(Tok::matching) else {
            return Err(Error::UnexpectedClosing(tok.into_char()));
        };
        if tok != expected {
            return Err(Error::Unmatched(expected.into_char(), tok.into_char()));
        }
    }
    if let Some(leftover) = stack.pop() {
        return Err(Error::UnexpectedEof(leftover.matching().into_char()));
    }

    Ok(())
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tok {
    LParen,
    RParen,
    LBracket,
    RBracket,
}

impl Tok {
    fn from_char(c: char) -> Result<Tok, Error> {
        Ok(match c {
            '(' => Tok::LParen,
            ')' => Tok::RParen,
            '[' => Tok::LBracket,
            ']' => Tok::RBracket,
            unexpected => return Err(Error::Unexpected(unexpected)),
        })
    }

    fn into_char(self) -> char {
        match self {
            Tok::LParen => '(',
            Tok::RParen => ')',
            Tok::LBracket => '[',
            Tok::RBracket => ']',
        }
    }

    fn opens(self) -> bool {
        // Do not use `matches!` due to the exhaustiveness check.
        match self {
            Tok::LParen | Tok::LBracket => true,
            Tok::RParen | Tok::RBracket => false,
        }
    }

    fn matching(self) -> Tok {
        match self {
            Tok::LParen => Tok::RParen,
            Tok::RParen => Tok::LParen,
            Tok::LBracket => Tok::RBracket,
            Tok::RBracket => Tok::LBracket,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Unexpected(char),
    UnexpectedClosing(char),
    Unmatched(/* expected */ char, /* instead got */ char),
    UnexpectedEof(/* expected */ char),
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        (@, $name:ident, $src:expr, $out:expr) => {
            #[test]
            fn $name() {
                assert_eq!(crate::validate($src), $out);
            }
        };
        ($name:ident, $src:expr) => {
            t!(@, $name, $src, Ok(()));
        };
        ($name:ident, $src:expr, $error:expr) => {
            t!(@, $name, $src, Err($error));
        };
    }

    t!(ok_1, "(()())");
    t!(ok_2, "((()))");
    t!(ok_3, "[[]([[]()])]");
    t!(unexpected, "*", Error::Unexpected('*'));
    t!(unopened, "())", Error::UnexpectedClosing(')'));
    t!(unmatched, "[)", Error::Unmatched(']', ')'));
    t!(expected_got_eof_1, "(", Error::UnexpectedEof(')'));
    t!(expected_got_eof_2, "([", Error::UnexpectedEof(']'));
}
