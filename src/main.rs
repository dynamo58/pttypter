use std::io::Read;

use pttypter::lexer::lex;
use pttypter::parse_args;
use pttypter::printer::{html, term, Output, ThemeKind};

// entry point of the program
fn main() {
    let args = parse_args().unwrap_or_else(|e| {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Error: failed to read input");

        if input != String::new() {
            term::print(lex(input), ThemeKind::Dark);
            std::process::exit(0);
        } else {
            println!("Error: {e}.");
            std::process::exit(1);
        }
    });

    match args.output {
        Some(Output::HTML) => html::print(lex(args.input), args.theme.unwrap()),
        _ => term::print(lex(args.input), args.theme.unwrap()),
    }
    std::process::exit(0);
}
