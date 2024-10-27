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
            assert_eq!(header.markdown(None).unwrap(), test_str);
        };

        run_test(1);
        run_test(2);
        run_test(3);
        run_test(4);
        run_test(5);
        run_test(6);
    }

    #[test]
    fn header_test02() {
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
                header.markdown(None).unwrap(),
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
    fn header_test03() {
        let text = "Hello, world! how are you? Great.";
        let mut test_str = text.to_owned();
        test_str.push_str("\n==\n");

        let (header, left) = Header::parse(&test_str).unwrap();
        assert_eq!(header.level(), 1);
        assert_eq!(header.text(), text);
        assert_eq!(header.alt_syntax, true);
        assert!(left.is_empty());
        assert_eq!(header.markdown(None).unwrap(), format!("{}\n==\n", text));

        let mut test_str = text.to_owned();
        test_str.push_str("\n--\n");
        let (header, left) = Header::parse(&test_str).unwrap();
        assert_eq!(header.level(), 2);
        assert_eq!(header.text(), text);
        assert_eq!(header.alt_syntax, true);
        assert!(left.is_empty());
        assert_eq!(header.markdown(None).unwrap(), format!("{}\n--\n", text));
    }
}
