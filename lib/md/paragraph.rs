use super::{MarkdownParser, Rule};
use pest::Parser;

pub struct Paragraph {
    text: String,
}

impl Paragraph {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
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
    fn paragraph_test01() {
        let (p, left) = Paragraph::parse("\nabc def\n").unwrap();
        assert_eq!(p.text(), "abc def");
        assert!(left.is_empty());

        let (p, left) = Paragraph::parse("\nabc.def\n").unwrap();
        assert_eq!(p.text(), "abc.def");
        assert!(left.is_empty());

        let (p, left) = Paragraph::parse("\na,;f.f\n").unwrap();
        assert_eq!(p.text(), "a,;f.f");
        assert!(left.is_empty());

        let (p, left) = Paragraph::parse("\nabc def\nghi").unwrap();
        assert_eq!(p.text(), "abc def");
        assert_eq!(left, "ghi");
    }
}
