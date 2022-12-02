use pttypter::lexer::lex;
use pttypter::parse_args;
use pttypter::printer::{html, term, Output, ThemeKind};

fn main() {
    let args = parse_args().unwrap_or_else(|e| {
        let mut input = String::new();
        std::io::stdin().lines().for_each(|l| {
            input.push_str(&l.unwrap_or_else(|_| {
                println!("Error: Failed to read input (also no arguments provided)");
                std::process::exit(1);
            }))
        });
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
