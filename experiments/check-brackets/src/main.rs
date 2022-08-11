use std::{
    fmt::{self, Display, Write as _},
    io::{self, Write},
};

fn check_brackets(input: &str) -> Result<(), Error> {
    let mut stack = Vec::new();

    for (position, char) in input.char_indices() {
        let bracket = Bracket::try_from(char).map_err(|_| Error {
            kind: ErrorKind::Unexpected(char),
            position,
        })?;

        if bracket.is_left() {
            stack.push(bracket);
            continue;
        }

        match stack.last() {
            Some(last) => {
                let expected = last.opposite();
                if bracket != expected {
                    return Err(Error {
                        kind: ErrorKind::ExpectedButGot(expected, bracket),
                        position,
                    });
                }
                stack.pop();
            }
            None => {
                return Err(Error {
                    kind: ErrorKind::ExpectedLeftGotRight,
                    position,
                })
            }
        }
    }

    if let Some(leftover) = stack.last() {
        return Err(Error {
            kind: ErrorKind::ExpectedButGotEof(leftover.opposite()),
            position: input.len(),
        });
    }

    Ok(())
}

#[derive(Debug, Copy, Clone)]
struct Error {
    position: usize,
    kind: ErrorKind,
}

impl Error {
    fn report(&self, original: &str, out: &mut dyn Write) -> io::Result<()> {
        writeln!(out, "Error: {}\n", self.kind)?;
        writeln!(out, "    | {original}")?;
        writeln!(out, "      {}^", " ".repeat(self.position))?;

        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
enum ErrorKind {
    ExpectedButGot(Bracket, Bracket),
    ExpectedButGotEof(Bracket),
    ExpectedLeftGotRight,
    Unexpected(char),
}

impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            ExpectedButGot(a, b) => write!(f, "expected `{a}`, but got `{b}`"),
            ExpectedButGotEof(a) => write!(f, "expected `{a}`, but got end of input"),
            ExpectedLeftGotRight => write!(f, "expected an opening bracket, but got a closing one"),
            Unexpected(a) => write!(f, "unexpected character `{a}`"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Bracket {
    LRound,
    RRound,
    LSquare,
    RSquare,
    LCurly,
    RCurly,
    LAngle,
    RAngle,
}

impl Bracket {
    pub fn is_left(&self) -> bool {
        use Bracket::*;
        match self {
            LRound | LSquare | LCurly | LAngle => true,
            _ => false,
        }
    }

    pub fn opposite(&self) -> Bracket {
        use Bracket::*;
        match self {
            LRound => RRound,
            RRound => LRound,
            LSquare => RSquare,
            RSquare => LSquare,
            LCurly => RCurly,
            RCurly => LCurly,
            LAngle => RAngle,
            RAngle => LAngle,
        }
    }
}

impl TryFrom<char> for Bracket {
    type Error = ();

    fn try_from(char: char) -> Result<Self, Self::Error> {
        use Bracket::*;
        Ok(match char {
            '(' => LRound,
            ')' => RRound,
            '[' => LSquare,
            ']' => RSquare,
            '{' => LCurly,
            '}' => RCurly,
            '<' => LAngle,
            '>' => RAngle,
            _ => return Err(()),
        })
    }
}

impl Display for Bracket {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        use Bracket::*;
        f.write_char(match self {
            LRound => '(',
            RRound => ')',
            LSquare => '[',
            RSquare => ']',
            LCurly => '{',
            RCurly => '}',
            LAngle => '<',
            RAngle => '>',
        })
    }
}

fn stdin_prompted_lines(prompt: &str) -> impl Iterator<Item = String> + '_ {
    std::iter::from_fn(move || {
        print!("{prompt}");
        io::stdout().flush().unwrap();

        let mut buf = String::new();
        match io::stdin().read_line(&mut buf).unwrap() {
            0 => None,
            _ => Some(buf.trim().into()),
        }
    })
}

fn main() {
    for line in stdin_prompted_lines("> ") {
        match check_brackets(&line) {
            Ok(_) => eprintln!("ok\n"),
            Err(error) => error.report(&line, &mut io::stderr()).unwrap(),
        }
    }
}
