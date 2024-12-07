use super::{MarkdownParser, MdOperation, Rule};
use crate::error::JTranslateError;
use crate::translate_text;
use error_stack::Result;
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

impl MdOperation for Header {
    fn text(&self) -> &str {
        &self.text
    }

    fn to_md_str(&self, translate: Option<(&str, &str)>) -> Result<String, JTranslateError> {
        let mut result = String::new();

        if !self.alt_syntax {
            result = (0..self.level).map(|_| '#').collect::<String>();
            result.push(' ');
        }

        if let Some((from, to)) = translate {
            let text = translate_text(&self.text, from, vec![to])?;
            result.push_str(text[0].text());
        } else {
            result.push_str(&self.text);
        }

        if self.alt_syntax {
            result.push('\n');
            if self.level == 1 {
                result.push_str("==");
            } else {
                result.push_str("--");
            }
        }

        result.push('\n');

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::Header;
    use super::MdOperation;

    #[test]
    fn header_1() {
        let run_test = |level: usize| {
            let mut test_str: String = (0..level).map(|_| '#').collect();
            test_str.push(' ');
            let text = "Hello, world! how are you? Great.";
            test_str.push_str(text);
            test_str.push('\n');

            let (header, left) = Header::parse(&test_str).unwrap();
            assert_eq!(header.level(), level);
            assert_eq!(header.text(), text);
            assert_eq!(header.alt_syntax, false);
            assert!(left.is_empty());
            assert_eq!(header.to_md_str(None).unwrap(), test_str);
        };

        run_test(1);
        run_test(2);
        run_test(3);
        run_test(4);
        run_test(5);
        run_test(6);
    }

    #[test]
    fn header_2() {
        let run_test = |level: usize| {
            let mut test_str: String = (0..level).map(|_| '#').collect();
            test_str.push(' ');
            let text = "Hello, world! how are you? Great.";
            test_str.push_str(text);
            test_str.push('\n');
            let extra = "DUMMY";
            test_str.push_str(extra);

            let (header, left) = Header::parse(&test_str).unwrap();
            assert_eq!(header.level(), level);
            assert_eq!(header.text(), text);
            assert_eq!(header.alt_syntax, false);
            assert_eq!(left, extra);
            assert_eq!(
                header.to_md_str(None).unwrap(),
                test_str.trim_end_matches(extra)
            );
        };

        run_test(1);
        run_test(2);
        run_test(3);
        run_test(4);
        run_test(5);
        run_test(6);
    }

    #[test]
    fn header_3() {
        let text = "Hello, world! how are you? Great.";
        let mut test_str = text.to_owned();
        test_str.push_str("\n==\n");

        let (header, left) = Header::parse(&test_str).unwrap();
        assert_eq!(header.level(), 1);
        assert_eq!(header.text(), text);
        assert_eq!(header.alt_syntax, true);
        assert!(left.is_empty());
        assert_eq!(header.to_md_str(None).unwrap(), format!("{}\n==\n", text));

        let mut test_str = text.to_owned();
        test_str.push_str("\n--\n");
        let (header, left) = Header::parse(&test_str).unwrap();
        assert_eq!(header.level(), 2);
        assert_eq!(header.text(), text);
        assert_eq!(header.alt_syntax, true);
        assert!(left.is_empty());
        assert_eq!(header.to_md_str(None).unwrap(), format!("{}\n--\n", text));
    }
}
