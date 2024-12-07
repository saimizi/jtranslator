use super::{MarkdownParser, MdOperation, Rule};
use jlogger_tracing::jdebug;
use pest::Parser;

pub struct LineBreak {}

impl LineBreak {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        jdebug!(input = input, func = "LineBreak::parse", line = line!());
        match MarkdownParser::parse(Rule::linebreak, input) {
            Ok(parsed) => {
                jdebug!(
                    parsed = parsed.as_str(),
                    func = "LineBreak::parse",
                    line = line!()
                );
                Some((LineBreak {}, &input[3..]))
            }
            _ => None,
        }
    }
}

impl MdOperation for LineBreak {}

pub struct Paragraph {
    text: String,
}

impl Paragraph {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        jdebug!(input = input, func = "Paragraph::parse()", line = line!());
        match MarkdownParser::parse(Rule::paragraph, input) {
            Ok(pair) => {
                let matched = pair.as_str();
                // matched include "\n\n"
                let index = matched.len() - 1;
                jdebug!(func = "Paragraph::parse", matched = matched, index = index);
                let left = &input[index..];
                let matched = matched.replace("\n", " ");
                jdebug!(
                    matched = matched,
                    func = "Paragraph::parse()",
                    line = line!(),
                    left = left
                );
                Some((
                    Self {
                        text: matched.trim().to_owned(),
                    },
                    left,
                ))
            }
            _ => None,
        }
    }
}

impl MdOperation for Paragraph {
    fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::LineBreak;
    use super::MdOperation;
    use super::Paragraph;

    #[allow(unused)]
    use jlogger_tracing::jdebug;

    #[test]
    fn linebreak_1() {
        let run_test = |test_str: &str| {
            let (lb, left) = LineBreak::parse(test_str).unwrap();
            assert_eq!(lb.text(), "");
            assert_eq!(left, &test_str[3..])
        };

        run_test("<br>");
        run_test("<br>a");
        run_test("  \n");
        run_test("  \na");
    }

    #[test]
    fn linebreak_2() {
        let run_test = |test_str: &str| {
            assert!(LineBreak::parse(test_str).is_none());
        };

        run_test("  a\n");
        run_test("   \na");
        run_test("    a");
    }

    #[test]
    fn paragraph_1() {
        let run_test = |test_str: &str, expected: &str, expected_left: &str| {
            let (p, left) = Paragraph::parse(test_str).unwrap();
            assert_eq!(p.text(), expected);
            assert_eq!(left, expected_left);
        };

        run_test("abc def\n\n", "abc def", "\n");
        run_test("abc.def\n\n", "abc.def", "\n");
        run_test("a,;f.f\n\n", "a,;f.f", "\n");
        run_test("abc  \ndef\n\n", "abc   def", "\n");
        run_test("abc def\n\nextra text", "abc def", "\nextra text");
        run_test(
            "2nd paragraph. *Italic*, **bold**, and `monospace`. Itemized lists\nlook like:\n\nextra text",
            "2nd paragraph. *Italic*, **bold**, and `monospace`. Itemized lists look like:", "\nextra text");
        run_test(
            "Use 3 dashes for an em-dash. Use 2 dashes for ranges (ex., \"it's all\nin chapters 12--14\"). Three dots ... will be converted to an ellipsis.\nUnicode is supported.\n\n",
            "Use 3 dashes for an em-dash. Use 2 dashes for ranges (ex., \"it's all in chapters 12--14\"). Three dots ... will be converted to an ellipsis. Unicode is supported.", "\n");
    }
}
