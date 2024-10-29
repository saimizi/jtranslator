use super::{MarkdownParser, Rule};
use jlogger_tracing::jdebug;
use pest::Parser;

pub struct Paragraph {
    text: String,
}

impl Paragraph {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        jdebug!(input = input, func = "Paragraph::parse()", line = line!());
        match MarkdownParser::parse(Rule::paragraph, input) {
            Ok(pair) => {
                let matched = pair.as_str();
                let index = matched.len();
                Some((
                    Self {
                        text: matched.trim().to_owned(),
                    },
                    &input[index..],
                ))
            }
            _ => None,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    use super::Paragraph;

    #[test]
    fn paragraph_1() {
        let run_test = |test_str: &str| {
            let (p, left) = Paragraph::parse(test_str).unwrap();
            let end = test_str.find("\n\n").unwrap();
            let text = test_str[..end].trim();
            let expected_left = &test_str[end + 2..];

            assert_eq!(p.text(), text);
            if expected_left.is_empty() {
                assert!(left.is_empty());
            } else {
                assert_eq!(left, expected_left);
            }
        };

        run_test("\nabc def\n\n");
        run_test("\nabc.def\n\n");
        run_test("\na,;f.f\n\n");
        run_test("\nabc def\n\nghi");
        run_test("2nd paragraph. *Italic*, **bold**, and `monospace`. Itemized lists\nlook like:\n\n"); 
    }
}
