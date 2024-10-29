use super::{MarkdownParser, Rule};
use jlogger_tracing::jdebug;
use pest::Parser;

pub struct Italic {
    text: String,
}

impl Italic {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        jdebug!(input = input, func = "Italic::parse()", line = line!());
        match MarkdownParser::parse(Rule::italic, input) {
            Ok(pair) => {
                let matched = pair.as_str();
                let index = matched.len();
                let text = matched[1..index - 1].to_owned();
                let left = &input[index..];
                jdebug!(
                    result = "succeed",
                    text = text,
                    left = left,
                    func = "Italic::parse()",
                    line = line!()
                );
                Some((Self { text }, left))
            }
            _ => {
                jdebug!(
                    result = "failed",
                    input = input,
                    func = "Italic::parse()",
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
    use super::Italic;
    #[allow(unused)]
    use jlogger_tracing::jdebug;

    #[test]
    fn italic_1() {
        let run_test = |test_str: &str| {
            let (p, left) = Italic::parse(test_str).unwrap();
            let end = if test_str.starts_with("*") {
                test_str[1..].find('*').unwrap() + 1
            } else if test_str.starts_with("_") {
                test_str[1..].find('_').unwrap() + 1
            } else {
                unreachable!();
            };

            let text = &test_str[1..end];
            let expected_left = &test_str[end + 1..];

            assert_eq!(p.text(), text);
            if expected_left.is_empty() {
                assert!(left.is_empty());
            } else {
                assert_eq!(left, expected_left);
            }
        };

        run_test("_abc_");
        run_test("*abc*");
        run_test("*abc*d");
        run_test("_abc_d");
        run_test("*abc**d");
        run_test("_abc__d");
    }

    #[test]
    fn italic_2() {
        assert!(Italic::parse("__").is_none());
        assert!(Italic::parse("**").is_none());
        assert!(Italic::parse("__a_").is_none());
        assert!(Italic::parse("**a*").is_none());
    }
}
