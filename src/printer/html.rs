// transcribing into HTML and CS

use crate::lexer::LexItem;
use crate::printer::{GRUVBOX_DARK, GRUVBOX_LIGHT};

use super::{ThemeKind, ToHTMLString};

static TEMPLATE: &'static str = r#"
	<style>
		.__pttypter_pre {
			font-family: monospace;
			font-size: inherit;
			display: block;
			background: none;
			white-space: pre;
			-webkit-overflow-scrolling: touch;
			overflow-x: scroll;
			max-width: 100%;
			min-width: 100px;
			padding: 0;
		}

		.__pttypter_pre {
			background-color: {{ BG }};
			color: {{ FG }};
		}

		.__pttypter__fg {
			color: {{ FG }};
		}

		.__pttypter__bg {
			color: {{ BG }};
		}

		.__pttypter__number {
			color: {{ NUMBER }};
		}

		.__pttypter__string {
			color: {{ STRING }};
		}

		.__pttypter__keyword {
			color: {{ KEYWORD }};
		}
	</style>

	<pre class="__pttypter_pre">{{ CODE }}</pre>
"#;

pub fn print(lexed: Vec<LexItem>, theme_kind: ThemeKind) {
    let mut json = String::new();

    let theme = match theme_kind {
        ThemeKind::Dark => &GRUVBOX_DARK,
        _ => &GRUVBOX_LIGHT,
    };

    let mut indent_num: usize = 0;
    for l in lexed {
        match l {
            LexItem::LBrace => {
                json.push_str("{\r\n");
                indent_num += 1;
                json.push_str("&nbsp;".repeat(2 * indent_num).as_str());
            }
            LexItem::RBrace => {
                indent_num -= 1;
                json.push_str(format!("\r\n{}}}", "&nbsp;".repeat(2 * indent_num)).as_str());
            }
            LexItem::LBracket => {
                json.push_str("[");
                indent_num += 1;
            }
            LexItem::RBracket => {
                json.push_str("]");
                indent_num -= 1;
            }
            LexItem::Quote => json.push_str("\""),
            LexItem::Colon => json.push_str(": "),
            LexItem::BraceComma => {
                json.push_str(format!(",\r\n{}", "&nbsp;".repeat(2 * indent_num)).as_str())
            }
            LexItem::BracketComma => json.push_str(", "),
            LexItem::Str(s) => {
                json.push_str(format!("<span class=\"__pttypter__string\">{s}</span>").as_str())
            }
            LexItem::Num(n) => {
                json.push_str(format!("<span class=\"__pttypter__number\">{n}</span>").as_str())
            }
            LexItem::Null => json.push_str("<span class=\"__pttypter__keyword\">null</span>"),
            LexItem::True => json.push_str("<span class=\"__pttypter__keyword\">true</span>"),
            LexItem::False => json.push_str("<span class=\"__pttypter__keyword\">false</span>"),
        }
    }

    print!(
        "\n{}",
        TEMPLATE
            .replace("{{ CODE }}", json.as_str())
            .replace("{{ BG }}", theme.bg.to_html_string().as_str())
            .replace("{{ FG }}", theme.fg.to_html_string().as_str())
            .replace("{{ NUMBER }}", theme.number.to_html_string().as_str())
            .replace("{{ KEYWORD }}", theme.keyword.to_html_string().as_str())
            .replace("{{ STRING }}", theme.string.to_html_string().as_str())
    )
}
