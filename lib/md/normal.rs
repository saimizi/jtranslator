use super::MdOperation;
use super::{MarkdownParser, Rule};
#[allow(unused)]
use jlogger_tracing::jdebug;
use pest::Parser;

pub struct NormalText {
    text: String,
}

impl NormalText {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        if let Ok(pair) = MarkdownParser::parse(Rule::normal, input) {
            let normal = pair.as_str();
            let left = &input[normal.len()..];
            Some((
                Self {
                    text: normal.trim().to_owned(),
                },
                left,
            ))
        } else {
            None
        }
    }
}

impl MdOperation for NormalText {
    fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::MdOperation;
    use super::NormalText;
    use jlogger_tracing::jdebug;

    #[test]
    fn normal_text_1() {
        let run_test = |test_str: &str, expected_left: &str| {
            let (parsed, left) = NormalText::parse(test_str).unwrap();
            assert_eq!(parsed.text(), test_str.trim());
            assert_eq!(left, expected_left);
        };

        run_test("This is a test", "");
        run_test(" This is a test ", "");
        run_test("1This is a test2", "");
        run_test("'#@;:,.'", "");
        run_test("1. abc", "");
        run_test("**a**", "");
        run_test("__a__", "");
        run_test("*_a_*", "");
        run_test("_*a*_", "");
    }

    #[test]
    fn normal_text_2() {
        let run_test = |test_str: &str| {
            assert!(NormalText::parse(test_str).is_none());
        };

        jdebug!(func = "normal_text_2", line = line!());
        run_test("1. abc\n");
        run_test("2. abc\n");
        run_test("* abc\n");
        run_test("- abc\n");
        run_test("+ abc\n");
        run_test("+ abc\n");
    }
}
