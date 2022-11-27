use pttypter::lexer::lex;

static TEST: &'static str = r#"
{
    "xd": 1,
    "dx": 2
}
"#;

fn main() {
    let lexed = lex(&TEST.to_owned());
    println!("{lexed:#?}");
    // let ast = parse(lexed);
    // println!("{ast:#?}");
}
