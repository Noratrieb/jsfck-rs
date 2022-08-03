use std::fmt::{self, Display, Formatter};
use std::io::{self, Read, Write};

fn main() -> anyhow::Result<()> {
    let stdin = std::io::stdin().lock();
    let mut stdin = io::BufReader::new(stdin);
    let stdout = std::io::stdout().lock();
    let mut stdout = io::BufWriter::new(stdout);

    let mut buf = vec![0; 4096];
    let mut string = String::with_capacity(8192);

    loop {
        let read = stdin.read(&mut buf)?;

        if read == 0 {
            return Ok(());
        }

        let next_data = &buf[0..read];

        js_fuck(next_data, &mut string)?;

        stdout.write_all(string.as_bytes())?;
        string.clear();
    }
}

fn js_fuck<W: fmt::Write>(code: &[u8], mut f: W) -> fmt::Result {
    write!(
        f,
        "(()=>{{}})[{}]({})()",
        FromString(b"constructor"),
        FromString(code)
    )
}

const ZERO: &str = "+[]";
const ONE: &str = "+!![]";

struct Number(u8);

impl Display for Number {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0 == 0 {
            f.write_str(ZERO)?;
            return Ok(());
        }
        let mut first = true;
        for _ in 0..self.0 {
            if !first {
                f.write_str(" + ")?;
            }
            first = false;
            f.write_str(ONE)?;
        }
        Ok(())
    }
}

struct FromString<'a>(&'a [u8]);

impl Display for FromString<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        js_fuck_from_string(self.0, f)
    }
}

fn js_fuck_from_string<W: fmt::Write>(str: &[u8], mut f: W) -> fmt::Result {
    let mut first = true;
    str.iter().try_for_each(|char| {
        if !first {
            f.write_str("+")?;
        }
        first = false;
        js_fuck_from_char(*char, &mut f)
    })
}

fn js_fuck_from_char<W: fmt::Write>(char: u8, mut f: W) -> fmt::Result {
    if js_fuck_trivial(char, &mut f)? {
        return Ok(());
    }

    write!(
        f,
        "([]+[])[{}][{}]({})",
        FromString(b"constructor"),
        FromString(b"fromCharCode"),
        Number(char)
    )
}

fn js_fuck_trivial<W: fmt::Write>(data: u8, mut f: W) -> Result<bool, fmt::Error> {
    match data {
        b'a' => write!(f, "(+{{}}+[])[{}]", Number(1)),
        b'b' => write!(f, "({{}}+[])[{}]", Number(2)),
        b'o' => write!(f, "({{}}+[])[{}]", Number(1)),
        b'e' => write!(f, "({{}}+[])[{}]", Number(4)),
        b'c' => write!(f, "({{}}+[])[{}]", Number(5)),
        b't' => write!(f, "({{}}+[])[{}]", Number(6)),
        b' ' => write!(f, "({{}}+[])[{}]", Number(7)),
        b'f' => write!(f, "(![]+[])[{}]", Number(0)),
        b's' => write!(f, "(![]+[])[{}]", Number(3)),
        b'r' => write!(f, "(!![]+[])[{}]", Number(1)),
        b'u' => write!(f, "(!![]+[])[{}]", Number(2)),
        b'i' => write!(f, "((+!![]/+[])+[])[{}]", Number(3)),
        b'n' => write!(f, "((+!![]/+[])+[])[{}]", Number(4)),

        b'S' => write!(
            f,
            "([]+([]+[])[{}])[{}]",
            FromString(b"constructor"),
            Number(9)
        ),
        b'g' => write!(
            f,
            "([]+([]+[])[{}])[{}]",
            FromString(b"constructor"),
            Number(14)
        ),
        b'p' => write!(
            f,
            "([]+(/-/)[{}])[{}]",
            FromString(b"constructor"),
            Number(14)
        ),
        b'\\' => write!(f, "(/\\\\/+[])[{}]", Number(1)),
        b'd' => write!(
            f,
            "({})[{}]({})",
            Number(13),
            FromString(b"toString"),
            Number(14)
        ),
        b'h' => write!(
            f,
            "({})[{}]({})",
            Number(17),
            FromString(b"toString"),
            Number(18)
        ),
        b'm' => write!(
            f,
            "({})[{}]({})",
            Number(22),
            FromString(b"toString"),
            Number(23)
        ),

        b'C' => write!(
            f,
            "((()=>{{}})[{}]({})()((/\\\\/+[])[{}]))[{}]",
            FromString(b"constructor"),
            FromString(b"return escape"),
            Number(1),
            Number(2)
        ),

        _ => return Ok(false),
    }?;
    Ok(true)
}
