pub mod header;
pub mod paragraph;

use std::fmt::Display;
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

#[cfg(test)]
mod tests {
    use super::{header::Header, paragraph::Paragraph};
    use jlogger_tracing::{jdebug, JloggerBuilder, LevelFilter};
    use pest::Parser;
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
    fn file_parse_test01() {
        let markdown = include_str!("../../test/markdown-sample.md");
        let (header, left) = Header::parse(&markdown).unwrap();
        jdebug!(
            header = header.text(),
            level = header.level(),
            alt_syntax = header.is_alt_syntax()
        );
        assert_eq!(header.text(), "An h1 header");
        assert_eq!(header.level(), 1);
        assert_eq!(header.is_alt_syntax(), false);

        let (p, left) = Paragraph::parse(&left).unwrap();
        jdebug!(paragraph = p.text(),);
        assert_eq!(p.text(), "This is the main style.");

        let (header, _left) = Header::parse(&left).unwrap();
        jdebug!(
            header = header.text(),
            level = header.level(),
            alt_syntax = header.is_alt_syntax()
        );
        assert_eq!(header.text(), "An h1 header");
        assert_eq!(header.level(), 1);
        assert_eq!(header.is_alt_syntax(), true);
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
