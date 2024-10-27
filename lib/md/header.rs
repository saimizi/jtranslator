use super::{MarkdownParser, Rule};
use jlogger_tracing::jdebug;
use pest::Parser;

pub struct Header {
    text: String,
    level: usize,
    alt_syntax: bool,
}

impl Header {
    pub fn level(&self) -> usize {
        self.level
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn is_alt_syntax(&self) -> bool {
        self.alt_syntax
    }

    pub fn parse(input: &str) -> Option<(Self, &str)> {
        if let Ok(pair) = MarkdownParser::parse(Rule::headings1, input) {
            let header = pair.as_str();
            let left = &input[header.len()..];
            let space = header.find(' ').unwrap();
            let level = header[0..space].len();
            let text = &header[space..];
            return Some((
                Self {
                    text: text.trim().to_owned(),
                    level,
                    alt_syntax: false,
                },
                left,
            ));
        }

        match MarkdownParser::parse(Rule::headings2, input) {
            Ok(pair) => {
                let header = pair.as_str();
                let left = &input[header.len()..];
                jdebug!(header = header, func = "Header::parse()", line = line!());
                let (text, mark) = header.trim().split_once('\n').unwrap();
                // Start with '==' is level1
                let mut level = 1;
                if mark.starts_with("--") {
                    level = 2;
                };

                Some((
                    Self {
                        text: text.trim().to_owned(),
                        level,
                        alt_syntax: true,
                    },
                    left,
                ))
            }
            _ => {
                jdebug!(input = input, func = "Header::parse()", line = line!());
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Header;

    #[test]
    fn header_test01() {
        let (header, left) = Header::parse("# abc\n").unwrap();
        assert_eq!(header.level(), 1);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert!(left.is_empty());

        let (header, left) = Header::parse("## abc\n").unwrap();
        assert_eq!(header.level(), 2);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert!(left.is_empty());

        let (header, left) = Header::parse("### abc\n").unwrap();
        assert_eq!(header.level(), 3);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert!(left.is_empty());

        let (header, left) = Header::parse("#### abc\n").unwrap();
        assert_eq!(header.level(), 4);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert!(left.is_empty());

        let (header, left) = Header::parse("##### abc\n").unwrap();
        assert_eq!(header.level(), 5);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert!(left.is_empty());

        let (header, left) = Header::parse("###### abc\n").unwrap();
        assert_eq!(header.level(), 6);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert!(left.is_empty());
    }

    #[test]
    fn header_test02() {
        let (header, left) = Header::parse("# abc\ndef").unwrap();
        assert_eq!(header.level(), 1);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert_eq!(left, "def");

        let (header, left) = Header::parse("## abc\ndef").unwrap();
        assert_eq!(header.level(), 2);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert_eq!(left, "def");

        let (header, left) = Header::parse("### abc\ndef").unwrap();
        assert_eq!(header.level(), 3);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert_eq!(left, "def");

        let (header, left) = Header::parse("#### abc\ndef").unwrap();
        assert_eq!(header.level(), 4);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert_eq!(left, "def");

        let (header, left) = Header::parse("##### abc\ndef").unwrap();
        assert_eq!(header.level(), 5);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert_eq!(left, "def");

        let (header, left) = Header::parse("###### abc\ndef").unwrap();
        assert_eq!(header.level(), 6);
        assert_eq!(header.text(), "abc");
        assert_eq!(header.alt_syntax, false);
        assert_eq!(left, "def");
    }

    #[test]
    fn header_test03() {
        use super::Header;

        let (header, left) = Header::parse("abc def\n==\n").unwrap();
        assert_eq!(header.level(), 1);
        assert_eq!(header.text(), "abc def");
        assert_eq!(header.alt_syntax, true);
        assert!(left.is_empty());

        let (header, left) = Header::parse("abc def\n--\n").unwrap();
        assert_eq!(header.level(), 2);
        assert_eq!(header.text(), "abc def");
        assert_eq!(header.alt_syntax, true);
        assert!(left.is_empty());
    }
}
