pub mod bold;
pub mod header;
pub mod italic;
pub mod item;
pub mod normal;
pub mod paragraph;
pub mod reference;

use crate::error::JTranslateError;
use crate::translate_text;
use error_stack::Result;
pub use header::Header;
pub use item::Item;
use jlogger_tracing::{jdebug, jerror, jinfo, JloggerBuilder, LevelFilter};
pub use paragraph::Paragraph;
use pest_derive::Parser;
pub use reference::MultipleLineRef;
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

pub trait MdOperation {
    fn text(&self) -> &str {
        ""
    }

    fn to_md_str(&self, translate: Option<(&str, &str)>) -> Result<String, JTranslateError> {
        let mut result = String::new();

        if let Some((from, to)) = translate {
            let text = translate_text(self.text(), from, vec![to])?;
            result.push_str(text[0].text());
        } else {
            result.push_str(self.text());
        }

        result.push('\n');

        Ok(result)
    }
}

pub enum MdEntry {
    Header(header::Header),
    Paragraph(paragraph::Paragraph),
    Item(item::Item),
    MultipleLineRef(reference::MultipleLineRef),
    NewLine,
}

pub fn md_parse(text: &str) -> Result<Vec<MdEntry>, JTranslateError> {
    let mut result = vec![];
    let mut text = text;
    loop {
        jdebug!(func = "md_parse", line = line!(), text = text);
        loop {
            if text.starts_with("\n") {
                result.push(MdEntry::NewLine);
                text = &text[1..];
            } else {
                break;
            }
        }

        if text.is_empty() {
            break;
        }

        if let Some((header, l)) = header::Header::parse(text) {
            result.push(MdEntry::Header(header));
            text = l;
            jdebug!(matched = "header", left = text);
            continue;
        } else {
            jdebug!(no_matched = "header", text = text);
        }

        if let Some((paragraph, l)) = paragraph::Paragraph::parse(text) {
            result.push(MdEntry::Paragraph(paragraph));
            text = l;
            jdebug!(matched = "paragraph", left = text);
            continue;
        } else {
            jdebug!(no_matched = "paragraph", text = text);
        }

        if let Some((item, l)) = item::Item::parse(text) {
            result.push(MdEntry::Item(item));
            text = l;
            jdebug!(matched = "item", left = text);
            continue;
        } else {
            jdebug!(no_matched = "item", text = text);
        }

        if let Some((reference, l)) = reference::MultipleLineRef::parse(text) {
            result.push(MdEntry::MultipleLineRef(reference));
            text = l;
            jdebug!(matched = "reference", left = text);
            continue;
        } else {
            jdebug!(no_matched = "reference", text = text);
        }

        panic!("invalid string: -{}-", text);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::md::item::Item;

    use super::MdOperation;
    use super::{header::Header, paragraph::Paragraph, reference::MultipleLineRef};
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
    fn file_parse_1() {
        let markdown = include_str!("../../test/markdown-sample.md");
        let (header, left) = Header::parse(&markdown).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            header = header.text(),
            level = header.level(),
            alt_syntax = header.is_alt_syntax()
        );
        assert_eq!(header.text(), "An h1 header");
        assert_eq!(header.level(), 1);
        assert_eq!(header.is_alt_syntax(), false);

        let (p, left) = Paragraph::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(paragraph = p.text(),);
        assert_eq!(p.text(), "This is the main style.");

        let (header, left) = Header::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            header = header.text(),
            level = header.level(),
            alt_syntax = header.is_alt_syntax()
        );
        assert_eq!(header.text(), "An h1 header");
        assert_eq!(header.level(), 1);
        assert_eq!(header.is_alt_syntax(), true);

        let (p, left) = Paragraph::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(paragraph = p.text(),);
        assert_eq!(p.text(), "Paragraphs are separated by a blank line.");

        let (p, left) = Paragraph::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(paragraph = p.text(),);
        assert_eq!(
            p.text(),
            "2nd paragraph. *Italic*, **bold**, and `monospace`. Itemized lists look like:"
        );

        let (p, left) = Item::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            item = p.text(),
            number_type = format!("{:?}", p.number_item()),
            level = p.level()
        );
        assert_eq!(p.text(), "this one");

        let (p, left) = Item::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            item = p.text(),
            number_type = format!("{:?}", p.number_item()),
            level = p.level()
        );
        assert_eq!(p.text(), "that one");

        let (p, left) = Item::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            item = p.text(),
            number_type = format!("{:?}", p.number_item()),
            level = p.level()
        );
        assert_eq!(p.text(), "the other one");

        let (p, left) = Item::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            item = p.text(),
            number_type = format!("{:?}", p.number_item()),
            level = p.level()
        );
        assert_eq!(p.text(), "first one");

        let (p, left) = Item::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            item = p.text(),
            number_type = format!("{:?}", p.number_item()),
            level = p.level()
        );
        assert_eq!(p.text(), "second one");

        let (p, left) = Item::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(
            item = p.text(),
            number_type = format!("{:?}", p.number_item()),
            level = p.level()
        );
        assert_eq!(p.text(), "third one");

        let (p, left) = Paragraph::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(paragraph = p.text());
        assert_eq!(
            p.text(),
            "Note that --- not considering the asterisk --- the actual text content starts at 4-columns in."
        );

        let (p, left) = MultipleLineRef::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(multi_line_ref = p.text());
        assert_eq!(
            p.text(),
            "Block quotes are > abc\nwritten like so.\n\nThey can span multiple paragraphs,\nif you like."
        );

        let (p, left) = Paragraph::parse(&left).unwrap();
        let left = left.trim_matches('\n');
        jdebug!(paragraph = p.text());
        assert_eq!(
        p.text(),
        "Use 3 dashes for an em-dash. Use 2 dashes for ranges (ex., \"it's all in chapters 12--14\"). Three dots ... will be converted to an ellipsis. Unicode is supported.");
    }

    #[test]
    fn words_1() {
        let run_test = |test_str| {
            let pair = MarkdownParser::parse(Rule::words, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str);
        };

        run_test("abc");
        run_test("ABC");
        run_test("Abc");
        run_test("abcdef");
        run_test("1 def");
        run_test("1 2");
        run_test("a1b 2bc");
        run_test("b b");
    }

    #[test]
    fn words_2() {
        let run_test = |test_str| {
            assert!(MarkdownParser::parse(Rule::words, test_str).is_err());
        };

        run_test("@");
        run_test("#");
        run_test("-");
    }

    #[test]
    fn header_marks_1() {
        let run_test = |test_str| {
            let pair = MarkdownParser::parse(Rule::MARK, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str);
        };

        run_test("#");
        run_test("+");
        run_test("-");
        run_test("@");
        run_test("_");
        run_test("/");
        run_test("\\");
        run_test("|");
        run_test("\"");
        run_test("\'");
        run_test("`");
        run_test("~");
        run_test("?");
        run_test(".");
        run_test(",");
        run_test(";");
    }

    #[test]
    fn headings1_1() {
        let run_test = |test_str| {
            jdebug!(test_str = test_str);
            let pair = MarkdownParser::parse(Rule::headings1, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str);
        };

        jdebug!(line = line!());
        run_test("# Hello\n");
        jdebug!(line = line!());
        run_test("# Hello, world!\n");
        jdebug!(line = line!());
        run_test("# Hello, world! how are you?\n");
        run_test("# Hello, world! how are you? Great.\n");
        run_test("# 1 Hello, world! how are you? Great.\n");
        run_test("# 1 2 Hello, world! how are you? Great.\n");
        run_test("# @ # ? - _ + - / \\ ~ , ; . |\n");
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

    #[test]
    fn normal_1() {
        let run_test = |test_str: &str, left: &str| {
            let pair = MarkdownParser::parse(Rule::normal, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str.trim_end_matches(left));
        };

        run_test("abc", "");
        run_test("1 2 Hello, world! how are you? Great", "");
        run_test("1 2 Hello, world! how are you? Great", "");
        run_test("1@2#Hello, \"world! 'how are you? Great", "");
        run_test("_1@2#Hello, \"world! 'how are you? Great", "");
        run_test("abc\n", "\n");
        run_test(" abc\n", "\n");
        run_test("**abc", "");
        run_test("**abc*", "");
        run_test("__abc", "");
        run_test("__abc_", "");
        run_test("abc**def**", "");
    }

    #[test]
    fn normal_2() {
        let run_test = |test_str| assert!(MarkdownParser::parse(Rule::normal, test_str).is_err());

        run_test("\nabc");
        run_test("\n\nabc");
    }

    #[test]
    fn non_number_item_1() {
        let run_test = |test_str: &str, left: &str| {
            let pair = MarkdownParser::parse(Rule::non_number_item, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str.trim_end_matches(left));
        };

        run_test("* item\n", "");
        run_test("   * item\n", "");
        run_test(" * item, hello world!\n", "");
        run_test(" * item, hello world! **abc**\n", "");
        run_test(" * item, hello world! __abc__\n", "");
        run_test("* abc*\n", "");
        run_test("* abc\nabc", "abc");

        run_test("- item\n", "");
        run_test("   - item\n", "");
        run_test(" - item, hello world!\n", "");
        run_test(" - item, hello world! __abc__\n", "");
        run_test("- abc-\n", "");
        run_test("- abc\nabc", "abc");
    }

    #[test]
    fn non_number_item_2() {
        let run_test = |test_str: &str| {
            assert!(MarkdownParser::parse(Rule::non_number_item, test_str).is_err());
        };

        run_test("*item\n");
        run_test("** item\n");
        run_test("* item");
        run_test("_item\n");
        run_test("__ item\n");
        run_test("_ item");
    }

    #[test]
    fn number_item_1() {
        let run_test = |test_str: &str, left: &str| {
            let pair = MarkdownParser::parse(Rule::number_item, test_str).unwrap();
            assert_eq!(pair.as_str(), test_str.trim_end_matches(left));
        };

        run_test("1. item\n", "");
        run_test("1. item.\n", "");
        run_test("1. Hello world! How are you? Great! \n", "");
        run_test("1. **Hello world**!\n", "");
        run_test("1. __Hello world__!\n", "");
        run_test("1. item.\n\nabc", "\nabc");
        run_test("  1. item.\n\nabc", "\nabc");
        run_test("  1. item.\n \nabc", " \nabc");
    }

    #[test]
    fn number_item_2() {
        let run_test = |test_str: &str| {
            assert!(MarkdownParser::parse(Rule::number_item, test_str).is_err());
        };

        run_test("1 item\n");
        run_test(" 1 item\n");
        run_test("a. item\n");
        run_test("a) item\n");
    }
}
