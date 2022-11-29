use std::str::FromStr;

pub mod html;
pub mod term;

#[derive(Debug)]
pub enum ThemeKind {
    Dark,
    Light,
}

impl FromStr for ThemeKind {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "dark" => Ok(Self::Dark),
            _ => Ok(Self::Light),
        }
    }
}

#[derive(Debug)]
pub enum Output {
    HTML,
    Terminal,
}

impl FromStr for Output {
    type Err = std::string::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.to_lowercase()[..] {
            "html" => Ok(Self::HTML),
            "html+css" => Ok(Self::HTML),
            _ => Ok(Self::Terminal),
        }
    }
}

type RGBTriplet = (u8, u8, u8);

pub trait ToHTMLString {
    fn to_html_string(self) -> String;
}

impl ToHTMLString for RGBTriplet {
    fn to_html_string(self) -> String {
        format!("rgb({}, {}, {})", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Theme {
    kind: ThemeKind,
    fg: RGBTriplet,
    bg: RGBTriplet,
    number: RGBTriplet,
    string: RGBTriplet,
    keyword: RGBTriplet,
}

pub static GRUVBOX_DARK: Theme = Theme {
    kind: ThemeKind::Dark,
    // bg: (40, 200, 40),
    bg: (40, 40, 40),
    number: (177, 98, 134),
    string: (215, 153, 33),
    fg: (235, 219, 178),
    keyword: (204, 35, 29),
};

pub static GRUVBOX_LIGHT: Theme = Theme {
    kind: ThemeKind::Light,
    bg: (251, 241, 199),
    number: (177, 98, 134),
    string: (215, 153, 33),
    fg: (60, 56, 54),
    keyword: (204, 35, 29),
};

// trait JSONPrinter {
// fn print(lexed: Vec<LexItem>, theme: ThemeKind);
// }
