use std::fmt::Display;

use jlogger_tracing::{jdebug, jinfo, JloggerBuilder};
use pest::{Parser, Token};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "../lib/md/markdown.pest"]
struct MarkdownParser;

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Rule::word => "word",
            Rule::words => "words",
            Rule::headings1 => "headings1",
            Rule::headings2 => "headings2",
            Rule::headings => "headings",
            _ => "...",
        };

        write!(f, "{msg}")
    }
}

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
        match MarkdownParser::parse(Rule::headings1, input) {
            Ok(pair) => {
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
            _ => {}
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

pub struct Paragraph {
    text: String,
}

impl Paragraph {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        match MarkdownParser::parse(Rule::paragraph, input) {
            Ok(pair) => {
                let matched = pair.as_str();
                let index = matched.len();
                Some((
                    Self {
                        text: matched.trim().to_owned(),
                    },
                    &input[index..],
                ))
            }
            _ => None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

//fn main() {
//    JloggerBuilder::new()
//        .max_level(jlogger_tracing::LevelFilter::TRACE)
//        .build();
//
//    let markdown = include_str!("../test/markdown-sample.md");
//    let (header, left) = Header::parse(&markdown).unwrap();
//    jinfo!(
//        header = header.text(),
//        level = header.level(),
//        alt_syntax = header.is_alt_syntax()
//    );
//
//    let (p, left) = Paragraph::parse(&left).unwrap();
//    jinfo!(paragraph = p.text(),);
//
//    let (header, _left) = Header::parse(&left).unwrap();
//    jinfo!(
//        header = header.text(),
//        level = header.level(),
//        alt_syntax = header.is_alt_syntax()
//    );
//}

#[cfg(test)]
mod tests {
    use jlogger_tracing::{jdebug, JloggerBuilder, LevelFilter};
    use pest::{Parser, Token};
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "../lib/md/markdown.pest"]
    struct MarkdownParser;

    #[ctor::ctor]
    fn setup() {
        JloggerBuilder::new()
            .log_file(Some(("/tmp/markdown.tmp", false)))
            .max_level(LevelFilter::DEBUG)
            .log_console(false)
            .build();
    }

    #[test]
    fn header_test01() {
        use super::Header;

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
        use super::Header;

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

    #[test]
    fn paragraph_test01() {
        use super::Paragraph;

        let (p, left) = Paragraph::parse("\nabc def\n").unwrap();
        assert_eq!(p.text(), "abc def");
        assert!(left.is_empty());

        let (p, left) = Paragraph::parse("\nabc.def\n").unwrap();
        assert_eq!(p.text(), "abc.def");
        assert!(left.is_empty());

        let (p, left) = Paragraph::parse("\na,;f.f\n").unwrap();
        assert_eq!(p.text(), "a,;f.f");
        assert!(left.is_empty());

        let (p, left) = Paragraph::parse("\nabc def\nghi").unwrap();
        assert_eq!(p.text(), "abc def");
        assert_eq!(left, "ghi");
    }

    #[test]
    fn word1() {
        let test_str = "abc";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "ABC";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "Abc";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "_abc";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn words1() {
        let test_str = "abc";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "abc def";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn words2() {
        let test_str = "1 def";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "1 2";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "a1b 2bc";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn words5() {
        let test_str = "_b";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "_b b_";
        let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn words6() {
        let test_str = "@";
        let result = MarkdownParser::parse(Rule::words, test_str);

        assert!(result.is_err());

        let test_str = "#";
        let result = MarkdownParser::parse(Rule::words, test_str);
        assert!(result.is_err());

        let test_str = "-";
        let result = MarkdownParser::parse(Rule::words, test_str);
        assert!(result.is_err());
    }

    #[test]
    fn headings1_1() {
        let test_str = "# abc\n";
        jdebug!("{}", test_str);
        let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "# abc\ndef";
        jdebug!("{}", test_str);
        let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
        assert_eq!(pair.as_str(), "# abc\n");

        let test_str = "# Abc\n";
        let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "# abc def\n";
        let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn headings1_2() {
        let test_str = "# 123\n";
        let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "# 123 456\n";
        let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn headings2_1() {
        let test_str = "abc\n==\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "abc def\n==\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "abc\n--\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "abc\n====\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);

        let test_str = "abc\n----\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str).unwrap();
        assert_eq!(pair.as_str(), test_str);
    }

    #[test]
    fn headings2_2() {
        let test_str = "abc\n===\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str);
        assert!(pair.is_err());

        let test_str = "abc\n---\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str);
        assert!(pair.is_err());

        let test_str = "abc\n==--\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str);
        assert!(pair.is_err());

        let test_str = "abc\n--==\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str);
        assert!(pair.is_err());

        let test_str = "a\nabc\n===\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str);
        assert!(pair.is_err());

        let test_str = "a\nabc\n---\n";
        let pair = MarkdownParser::parse(Rule::headings2, test_str);
        assert!(pair.is_err());
    }
}
