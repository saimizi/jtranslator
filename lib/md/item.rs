use std::usize;

use super::MdOperation;
#[allow(unused)]
use super::{MarkdownParser, Rule};
#[allow(unused)]
use jlogger_tracing::jdebug;
#[allow(unused)]
use pest::Parser;
use pest::Token;

#[allow(unused)]
pub struct Item {
    text: String,
    mark: Option<char>,
    level: usize,
}

impl Item {
    pub fn parse(input: &str) -> Option<(Self, &str)> {
        if let Ok(pair) = MarkdownParser::parse(Rule::list, input) {
            let mut item_end = 0_usize;
            let mut mark_start = 0_usize;
            let mut mark_end = 0_usize;
            let mut is_number_item = false;

            for token in pair.tokens() {
                match token {
                    Token::Start { rule, pos } => {
                        if rule == Rule::non_number_item_mark {
                            mark_start = pos.pos();
                        }

                        if rule == Rule::number_item_mark {
                            mark_start = pos.pos();
                        }
                    }
                    Token::End { rule, pos } => match rule {
                        Rule::number_item => {
                            item_end = pos.pos();
                            is_number_item = true;
                        }
                        Rule::non_number_item => {
                            item_end = pos.pos();
                        }
                        Rule::non_number_item_mark => {
                            mark_end = pos.pos();
                        }
                        Rule::number_item_mark => {
                            mark_end = pos.pos();
                        }
                        _ => {}
                    },
                }
            }

            let left = &input[item_end..];
            let level = input[..mark_start].len();
            let mark = if !is_number_item {
                Some(input.chars().collect::<Vec<char>>()[mark_start])
            } else {
                None
            };
            let text = input[mark_end..].trim().to_owned();

            Some((Self { text, mark, level }, left))
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
