#[derive(Debug, Clone)]
enum GrammarItem {
    Brace,
    Bracket,
    Quote,
    Colon,
    Comma,
    Str(String),
    // to serve arbitrarily large numbers (and not have to care about
    // ints/floats), internally numbers are gonna be represented as strings and
    // only tampered with during output
    Num(String),
    Bool(bool),
    Null,
}

struct Node {
    carrier: GrammarItem,
    children: Vec<Node>,
}

impl Node {
    pub fn new() -> Node {
        Node {
            carrier: GrammarItem::Brace,
            children: Vec::new(),
        }
    }
}
