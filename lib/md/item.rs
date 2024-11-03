use super::MdOperation;
#[allow(unused)]
use super::{MarkdownParser, Rule};
#[allow(unused)]
use jlogger_tracing::jdebug;
#[allow(unused)]
use pest::Parser;

#[allow(unused)]
pub struct Item {
    element: Vec<Box<dyn MdOperation>>,
    mark: Option<char>,
    level: usize,
}

impl Item {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        if let Ok(pair) = MarkdownParser::parse(Rule::list, input) {
            let left = &input[pair.as_str().len()..];

            Some((
                Self {
                    element: Vec::new(),
                    mark: None,
                    level: 0,
                },
                left,
            ))
        } else {
            None
        }
    }

    pub fn number_item(&self) -> bool {
        self.mark.is_none()
    }

    pub fn level(&self) -> usize {
        self.level
    }
}

impl MdOperation for Item {}
