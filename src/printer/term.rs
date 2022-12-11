// printing into terminal

use ansi_term::Color;

use super::ThemeKind;
use crate::lexer::LexItem;
use crate::printer::{GRUVBOX_DARK, GRUVBOX_LIGHT};

// just a wrapper to make the upcoming code a bit cleaner
// theme background is supressed when outputting to terminal
fn col(fg: (u8, u8, u8), input: &str) {
    print!("{}", Color::RGB(fg.0, fg.1, fg.2).paint(input));
}

pub fn print(lexed: Vec<LexItem>, theme_kind: ThemeKind) {
    let theme = match theme_kind {
        ThemeKind::Dark => &GRUVBOX_DARK,
        _ => &GRUVBOX_LIGHT,
    };

    let mut indent_num: usize = 0;
    for l in lexed {
        match l {
            LexItem::LBrace => {
                col(theme.fg, "{\r\n");
                indent_num += 1;
                col(theme.fg, "  ".repeat(indent_num).as_str());
            }
            LexItem::RBrace => {
                indent_num -= if indent_num > 0 { 1 } else { 0 };
                col(
                    theme.fg,
                    format!("\r\n{}", "  ".repeat(indent_num)).as_str(),
                );
                col(theme.fg, "}");
            }
            LexItem::LBracket => {
                col(theme.fg, "[");
                indent_num += 1;
            }
            LexItem::RBracket => {
                col(theme.fg, "]");
                indent_num -= if indent_num > 0 { 1 } else { 0 };
            }
            LexItem::Quote => col(theme.fg, "\""),
            LexItem::Colon => col(theme.fg, ": "),
            LexItem::BraceComma => {
                col(theme.fg, ",");
                col(
                    theme.fg,
                    format!("\r\n{}", "  ".repeat(indent_num)).as_str(),
                );
            }
            LexItem::BracketComma => col(theme.fg, ", "),
            LexItem::Str(s) => col(theme.string, s.as_str()),
            LexItem::Num(n) => col(theme.number, n.as_str()),
            LexItem::Null => col(theme.keyword, "null"),
            LexItem::True => col(theme.keyword, "True"),
            LexItem::False => col(theme.keyword, "False"),
        }
    }
    print!("\n")
}
