pub mod header;
pub mod paragraph;

use pest_derive::Parser;
use std::fmt::Display;

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
        let run_test = |test_str| {
            let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str);
        };

        run_test("abc");
        run_test("ABC");
        run_test("Abc");
        run_test("abc def");
        run_test("1 def");
        run_test("1 2");
        run_test("a1b 2bc");
        run_test("_b b_");
    }

    #[test]
    fn words2() {
        let run_test = |test_str| {
            assert!(MarkdownParser::parse(Rule::words, test_str).is_err());
        };

        run_test("@");
        run_test("#");
        run_test("-");
    }

    #[test]
    fn headings1_1() {
        let run_test = |test_str| {
            let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str);
        };

        run_test("# Hello\n");
        run_test("# Hello, world!\n");
        run_test("# Hello, world! how are you?\n");
        run_test("# Hello, world! how are you? Great.\n");
        run_test("# 1 Hello, world! how are you? Great.\n");
        run_test("# 1 2 Hello, world! how are you? Great.\n");
    }

    #[test]
    fn headings2_1() {
        let run_test = |test_str| {
            let pair = MarkdownParser::parse(Rule::headings2, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str);
        };

        run_test("Hello\n==\n");
        run_test("Hello\n--\n");
        run_test("Hello, world!\n==\n");
        run_test("Hello, world!\n--\n");
        run_test("Hello, world! how are you?\n==\n");
        run_test("Hello, world! how are you?\n--\n");
        run_test("Hello, world! how are you? Great.\n==\n");
        run_test("Hello, world! how are you? Great.\n--\n");
        run_test("1 Hello, world! how are you? Great.\n==\n");
        run_test("1 Hello, world! how are you? Great.\n--\n");
        run_test("1 2 Hello, world! how are you? Great.\n==\n");
        run_test("1 2 Hello, world! how are you? Great.\n--\n");
        run_test("1 2 Hello, world! how are you? Great.\n====\n");
        run_test("1 2 Hello, world! how are you? Great.\n----\n");
    }

    #[test]
    fn headings2_2() {
        let run_test = |test_str| {
            assert!(MarkdownParser::parse(Rule::headings2, test_str).is_err());
        };

        run_test("1 2 Hello, world! how are you? Great.\n===\n");
        run_test("1 2 Hello, world! how are you? Great.\n---\n");
        run_test("1 2 Hello, world! how are you? Great.\n==--\n");
        run_test("1 2 Hello, world! how are you? Great.\n--==\n");
    }
}
