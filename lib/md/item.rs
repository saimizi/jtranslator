use std::usize;

use super::MdOperation;
#[allow(unused)]
use super::{MarkdownParser, Rule};
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
        if let Ok(pair) = MarkdownParser::parse(Rule::item, input) {
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
            jdebug!(
                mark_start = mark_start,
                func = "Item::parse()",
                line = line!()
            );
            let level = (input[..mark_start].len() / 2) + 1;
            let mark = if !is_number_item {
                Some(input.chars().collect::<Vec<char>>()[mark_start])
            } else {
                None
            };
            let text = input[mark_end..item_end].trim().to_owned();

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

    pub fn mark(&self) -> Option<char> {
        self.mark
    }
}

impl MdOperation for Item {
    fn text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
pub mod test {
    use super::Item;
    use super::MdOperation;

    #[test]
    fn item_1() {
        let run_test = |test_str: &str,
                        expect_text: &str,
                        expect_mark: Option<char>,
                        expect_level: usize,
                        expect_left: &str| {
            let (item, left) = Item::parse(test_str).unwrap();
            assert_eq!(item.text(), expect_text);
            assert_eq!(item.mark(), expect_mark);
            assert_eq!(item.level(), expect_level);
            assert_eq!(left, expect_left);
        };

        run_test(
            "1. This is item of level 1.\n",
            "This is item of level 1.",
            None,
            1,
            "",
        );
        run_test(
            "  1. This is item of level 2.\n",
            "This is item of level 2.",
            None,
            2,
            "",
        );
        run_test(
            "    1. This is item of level 3.\n",
            "This is item of level 3.",
            None,
            3,
            "",
        );
        run_test(
            "    1. This is item of level 3.\nabc",
            "This is item of level 3.",
            None,
            3,
            "abc",
        );
        run_test(
            "2. This is item of level 1.\n",
            "This is item of level 1.",
            None,
            1,
            "",
        );
        run_test(
            "  2. This is item of level 2.\n",
            "This is item of level 2.",
            None,
            2,
            "",
        );
        run_test(
            "    3. This is item of level 3.\n",
            "This is item of level 3.",
            None,
            3,
            "",
        );
        run_test(
            "* This is item of level 1.\n",
            "This is item of level 1.",
            Some('*'),
            1,
            "",
        );
        run_test(
            "  * This is item of level 2.\n",
            "This is item of level 2.",
            Some('*'),
            2,
            "",
        );
        run_test(
            "    * This is item of level 3.\n",
            "This is item of level 3.",
            Some('*'),
            3,
            "",
        );
        run_test(
            "- This is item of level 1.\n",
            "This is item of level 1.",
            Some('-'),
            1,
            "",
        );
        run_test(
            "  - This is item of level 2.\n",
            "This is item of level 2.",
            Some('-'),
            2,
            "",
        );
        run_test(
            "    - This is item of level 3.\n",
            "This is item of level 3.",
            Some('-'),
            3,
            "",
        );
        run_test(
            "+ This is item of level 1.\n",
            "This is item of level 1.",
            Some('+'),
            1,
            "",
        );
        run_test(
            "  + This is item of level 2.\n",
            "This is item of level 2.",
            Some('+'),
            2,
            "",
        );
        run_test(
            "    + This is item of level 3.\n",
            "This is item of level 3.",
            Some('+'),
            3,
            "",
        );
    }
}
