use super::{MarkdownParser, Rule};
use jlogger_tracing::jdebug;
use pest::Parser;

pub struct Bold {
    text: String,
}

impl Bold {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        jdebug!(input = input, func = "bold::parse()", line = line!());
        match MarkdownParser::parse(Rule::bold, input) {
            Ok(pair) => {
                let matched = pair.as_str();
                let index = matched.len();
                let text = matched[2..index - 2].to_owned();

                let left = &input[matched.len()..];
                jdebug!(
                    result = "succeed",
                    text = text,
                    left = left,
                    func = "bold::parse()",
                    line = line!()
                );
                Some((Self { text }, left))
            }
            _ => {
                jdebug!(
                    result = "failed",
                    input = input,
                    func = "bold::parse()",
                    line = line!()
                );
                None
            }
        }
    }

    pub fn text(&self) -> &str {
        self.text.trim_matches('*')
    }
}

#[cfg(test)]
mod tests {
    use super::Bold;
    #[allow(unused)]
    use jlogger_tracing::jdebug;

    #[test]
    fn bold_1() {
        let run_test = |test_str: &str| {
            let (p, left) = Bold::parse(test_str).unwrap();
            let end = if test_str.starts_with("**") {
                test_str[2..].find("**").unwrap() + 2
            } else if test_str.starts_with("__") {
                test_str[2..].find("__").unwrap() + 2
            } else {
                unreachable!();
            };

            let expected_text = &test_str[2..end];
            let expected_left = &test_str[end + 2..];
            jdebug!(expected_text = expected_text, expected_left = expected_left);

            assert_eq!(p.text(), expected_text);
            if expected_left.is_empty() {
                assert!(left.is_empty());
            } else {
                assert_eq!(left, expected_left);
            }
        };

        run_test("__abc__");
        run_test("**abc**");
        run_test("**abc**d");
        run_test("__abc__d");
        run_test("**abc***d");
        run_test("__abc___d");
    }

    #[test]
    fn bold_2() {
        assert!(Bold::parse("____").is_none());
        assert!(Bold::parse("****").is_none());
        assert!(Bold::parse("____a_").is_none());
        assert!(Bold::parse("****a*").is_none());
    }
}
