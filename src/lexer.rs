#[derive(Debug, PartialEq)]
pub enum LexItem {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Quote,
    Colon,
    BraceComma,
    BracketComma,
    Str(String),
    Num(String),
    Null,
    True,
    False,
}

// it was a stupid decision to do it this way, but im too lazy to refactor it rn
#[derive(PartialEq, Debug)]
pub enum Environment {
    Scope,            // {# ...}
    ItemNameInside,   // {... "#"}
    PostItemName,     // {"..."#}
    PostPostItemName, // {"...": #}
    ItemValueInside,  // {"...": "#"} or {"...": 621#.126}
    ItemValueToken,   // {"...": .#.}
    PostItemValue,    // {"...": ...#}
    ListScope,        // {"...": [#]}
}

// used when a non-string value is to be appended to the lex stack
fn infer_token(s: &String) -> LexItem {
    match s.as_str() {
        "True" => LexItem::True,
        "False" => LexItem::False,
        "null" => LexItem::Null,
        x => LexItem::Num(x.to_owned()),
    }
}

// turn the input into well-defined lexed objects
pub fn lex(input: String) -> Vec<LexItem> {
    let mut result = Vec::new();
    let mut char_bank = String::new();
    let mut env: Option<Environment> = None;

    for c in input.chars() {
        match c {
            '{' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                Some(Environment::PostPostItemName) | None => {
                    env = Some(Environment::Scope);
                    result.push(LexItem::LBrace);
                }
                _ => (),
            },
            '}' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                Some(Environment::ItemValueToken) => {
                    if char_bank.len() > 0 {
                        result.push(infer_token(&char_bank));
                        char_bank = String::new();
                    }
                    result.push(LexItem::RBrace);
                    env = Some(Environment::Scope);
                }
                Some(Environment::PostItemValue) | Some(Environment::Scope) => {
                    if char_bank.len() > 0 {
                        result.push(LexItem::Str(char_bank));
                        char_bank = String::new();
                    }
                    result.push(LexItem::RBrace);
                    env = Some(Environment::Scope);
                }
                _ => (),
            },
            '"' => match env {
                Some(Environment::ItemNameInside) => {
                    result.push(LexItem::Str(char_bank));
                    result.push(LexItem::Quote);
                    env = Some(Environment::PostItemName);
                    char_bank = String::new();
                }
                Some(Environment::ItemValueInside) => {
                    result.push(LexItem::Str(char_bank));
                    char_bank = String::new();
                    result.push(LexItem::Quote);
                    env = Some(Environment::PostItemValue);
                }
                Some(Environment::Scope) => {
                    result.push(LexItem::Quote);
                    env = Some(Environment::ItemNameInside);
                }
                Some(Environment::PostPostItemName) => {
                    result.push(LexItem::Quote);
                    env = Some(Environment::ItemValueInside);
                }
                _ => (),
            },
            ':' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                Some(Environment::PostItemName) => {
                    result.push(LexItem::Colon);
                    env = Some(Environment::PostPostItemName);
                }
                _ => (),
            },
            ',' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                Some(Environment::PostItemValue) | Some(Environment::Scope) => {
                    if char_bank.len() > 0 {
                        result.push(LexItem::Str(char_bank));
                        char_bank = String::new();
                    }

                    result.push(LexItem::BraceComma);
                    env = Some(Environment::Scope);
                }
                Some(Environment::ItemValueToken) => {
                    if char_bank.len() > 0 {
                        result.push(infer_token(&char_bank));
                        char_bank = String::new();
                    }

                    result.push(LexItem::BraceComma);
                    env = Some(Environment::Scope);
                }
                Some(Environment::ListScope) => {
                    if char_bank.len() > 0 {
                        result.push(infer_token(&char_bank));
                        char_bank = String::new();
                    }
                    result.push(LexItem::BracketComma);
                    env = Some(Environment::ListScope);
                }
                _ => (),
            },
            '[' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                Some(Environment::PostPostItemName) => {
                    result.push(LexItem::LBracket);
                    env = Some(Environment::ListScope);
                }
                _ => (),
            },
            ']' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                Some(Environment::ListScope) => {
                    if char_bank.len() > 0 {
                        result.push(infer_token(&char_bank));
                        char_bank = String::new();
                    }

                    result.push(LexItem::RBracket);
                    env = Some(Environment::PostItemValue);
                }
                _ => (),
            },
            '\n' | '\t' | '\r' => {
                continue;
            }
            ' ' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                }
                _ => {
                    continue;
                }
            },
            _ => {
                if env == Some(Environment::PostPostItemName) {
                    env = Some(Environment::ItemValueToken);
                }
                char_bank.push(c);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::lex;
    use super::LexItem;

    fn assert_vecs<T: PartialEq>(v1: Vec<T>, v2: Vec<T>) {
        assert!(v1.len() == v2.len());
        for i in 0..v1.len() {
            assert!(v1[i] == v2[i]);
        }
    }

    #[test]
    fn t() {
        assert_vecs(lex("{}".into()), vec![LexItem::LBrace, LexItem::RBrace]);
    }
}
