#[derive(Debug, PartialEq)]
pub enum LexItem {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Quote,
    Colon,
    Comma,
    Str(String),
    Num(String),
}

#[derive(PartialEq, Debug)]
pub enum Environment {
    Scope,            // {# ...}
    ItemNameInside,   // {... "#"}
    PostItemName,     // {"..."#}
    PostPostItemName, // {"...": #}
    ItemValueInside,  // {"...": "#"} or {"...": 621#.126}
    ItemValueNumber,  // {"...": .#.}
    PostItemValue,    // {"...": ...#}
}

pub fn lex(input: &String) -> Result<Vec<LexItem>, String> {
    let mut result = Vec::new();

    let mut char_bank: Vec<char> = Vec::new();
    let mut env: Option<Environment> = None;

    let mut it = input.chars().peekable();
    while let Some(&c) = it.peek() {
        // println!("processing char {c}");
        match c {
            '{' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                    it.next();
                }
                None | Some(Environment::PostPostItemName) => {
                    env = Some(Environment::Scope);
                    result.push(LexItem::LBrace);
                    it.next();
                }
                _ => {
                    dbg!(&env, &c);
                }
            },
            '}' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                    it.next();
                }
                Some(Environment::ItemValueNumber) => {
                    if char_bank.len() > 0 {
                        result.push(LexItem::Num(String::from_iter(char_bank)));
                        char_bank = vec![];
                    }
                    result.push(LexItem::RBrace);
                    it.next();
                }
                Some(Environment::PostItemValue) | Some(Environment::Scope) => {
                    if char_bank.len() > 0 {
                        result.push(LexItem::Str(String::from_iter(char_bank)));
                        char_bank = vec![];
                    }
                    result.push(LexItem::RBrace);
                    it.next();
                }
                _ => {
                    dbg!(&env, &c);
                }
            },
            '"' => match env {
                Some(Environment::ItemNameInside) => {
                    result.push(LexItem::Str(String::from_iter(char_bank)));
                    result.push(LexItem::Quote);
                    env = Some(Environment::PostItemName);
                    char_bank = vec![];
                    it.next();
                }
                Some(Environment::ItemValueInside) => {
                    result.push(LexItem::Str(String::from_iter(char_bank)));
                    env = Some(Environment::PostItemValue);
                    char_bank = vec![];
                    it.next();
                }
                Some(Environment::Scope) => {
                    result.push(LexItem::Quote);
                    env = Some(Environment::ItemNameInside);
                    it.next();
                }
                _ => {
                    dbg!(&env, &c);
                }
            },
            ':' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                    it.next();
                }
                Some(Environment::PostItemName) => {
                    result.push(LexItem::Colon);
                    env = Some(Environment::PostPostItemName);
                    it.next();
                }
                _ => {
                    dbg!(&env, &c);
                }
            },
            ',' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                    it.next();
                }
                Some(Environment::PostItemValue) => {
                    if char_bank.len() > 0 {
                        result.push(LexItem::Str(String::from_iter(char_bank)));
                        char_bank = vec![];
                    }

                    result.push(LexItem::Comma);
                    env = Some(Environment::Scope);
                    it.next();
                }
                Some(Environment::ItemValueNumber) => {
                    if char_bank.len() > 0 {
                        result.push(LexItem::Num(String::from_iter(char_bank)));
                        char_bank = vec![];
                    }

                    result.push(LexItem::Comma);
                    env = Some(Environment::Scope);
                    it.next();
                }
                _ => {
                    dbg!(&env, &c);
                }
            },
            // '[' => match env {
            //     Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
            //         char_bank.push(c);
            //         it.next();
            //     }
            //     Some(Environment::Scope) => {
            //         result.push(LexItem::LBracket);
            //         env = Some(Environment::ItemNameInside);
            //         it.next();
            //     }
            //     _ => {
            //         panic!("Unexpected environment: {env:?}");
            //     }
            // },
            '\n' => {
                it.next();
                continue;
            }
            '\t' => {
                it.next();
                continue;
            }
            ' ' => match env {
                Some(Environment::ItemValueInside) | Some(Environment::ItemNameInside) => {
                    char_bank.push(c);
                    it.next();
                }
                _ => {
                    it.next();
                    continue;
                }
            },
            _ => {
                match env {
                    Some(Environment::PostPostItemName) => {
                        env = Some(Environment::ItemValueNumber);
                    }
                    _ => (),
                }
                char_bank.push(c);
                it.next();
            }
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::lex;
    use super::LexItem;

    fn match_vecs<T: PartialEq>(v1: Vec<T>, v2: Vec<T>) {
        assert!(v1.len() == v2.len());
        for i in 0..v1.len() {
            assert!(v1[i] == v2[i]);
        }
    }

    #[test]
    fn t() {
        match_vecs(
            lex(&String::from("{}")).unwrap(),
            vec![LexItem::LBrace, LexItem::RBrace],
        );
    }
}
