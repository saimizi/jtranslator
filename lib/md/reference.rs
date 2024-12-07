use super::translate_text;
use super::MdOperation;
#[allow(unused)]
use super::{MarkdownParser, Rule};
use jlogger_tracing::jdebug;
#[allow(unused)]
use pest::Parser;
use pest::Token;

#[allow(unused)]
pub struct SingleLineRef {
    text: String,
}

impl SingleLineRef {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        jdebug!(input = input);
        if let Ok(pair) = MarkdownParser::parse(Rule::single_ref, input) {
            let mut end = 0_usize;
            let mut mark_end = 0_usize;

            for token in pair.tokens() {
                if let Token::End { rule, pos } = token {
                    if rule == Rule::single_ref_mark {
                        mark_end = pos.pos();
                    }

                    if rule == Rule::single_ref {
                        end = pos.pos();
                    }
                }
            }

            let left = if input.as_bytes().get(end + 1).is_some() {
                &input[end..]
            } else {
                ""
            };

            jdebug!(input = input, end = end, left = left);
            let text = input[mark_end..end].trim().to_owned();

            return Some((SingleLineRef { text }, left));
        }

        None
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}

pub struct MultipleLineRef {
    buf: String,
}

impl MultipleLineRef {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        let mut left = input;
        let mut buf = String::new();
        loop {
            if let Some((single_ref, l)) = SingleLineRef::parse(left) {
                jdebug!(single_ref = single_ref.text());
                buf.push_str(single_ref.text());
                buf.push('\n');
                left = l;
            } else {
                jdebug!(input = left, "break");

                break;
            }
        }

        buf = buf.trim().to_owned();

        if buf.is_empty() {
            None
        } else {
            Some((MultipleLineRef { buf }, left))
        }
    }
}

impl MdOperation for MultipleLineRef {
    fn text(&self) -> &str {
        &self.buf
    }

    fn to_md_str(
        &self,
        translate: Option<(&str, &str)>,
    ) -> error_stack::Result<String, crate::error::JTranslateError> {
        let mut result = String::from("> ");
        if let Some((from, to)) = translate {
            let text = translate_text(self.text(), from, vec![to])?;
            result.push_str(text[0].text());
        } else {
            result.push_str(self.text());
        }
        let mut result = result.replace("\n", "\n> ");
        result.push('\n');
        Ok(result)
    }
}

#[cfg(test)]
pub mod test {
    use super::MdOperation;
    use super::MultipleLineRef;
    use super::SingleLineRef;

    #[test]
    fn single_line_ref_1() {
        let run_test = |test_str: &str, expect_text: &str, expect_left: &str| {
            let (single_ref, left) = SingleLineRef::parse(test_str).unwrap();
            assert_eq!(single_ref.text(), expect_text);
            assert_eq!(left, expect_left);
        };

        run_test("> reference 1\n", "reference 1", "");
        run_test(">reference 2\n", "reference 2", "");
        run_test("  >reference 3\n", "reference 3", "");
        run_test("  > reference 4\n", "reference 4", "");
        run_test("  > reference state.\n", "reference state.", "");
    }

    #[test]
    fn multiple_line_ref_1() {
        let run_test = |test_str: &str, expect_text: &str, expect_left: &str| {
            let (multi_line_ref, left) = MultipleLineRef::parse(test_str).unwrap();
            assert_eq!(multi_line_ref.text(), expect_text);
            assert_eq!(left.trim(), expect_left);
        };

        let text_str = "> First line\n> Second line\n";

        run_test(text_str, "First line\nSecond line", "");
    }
}
