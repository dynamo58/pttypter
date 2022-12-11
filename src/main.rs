use std::io::Read;

use pttypter::lexer::lex;
use pttypter::parse_args;
use pttypter::printer::{html, term, Output};

// entry point of the program
fn main() {
    // parse CLI args
    let mut args = parse_args().unwrap_or_else(|e| {
        println!("Error: {e}");
        std::process::exit(1);
    });

    // if there is no JSON input, try reading one from STDIN
    if args.input == Some(String::new()) || args.input == None {
        let mut input = String::new();
        std::io::stdin()
            .read_to_string(&mut input)
            .expect("Error: failed to read input");

        args.input = Some(input);
    }

    // envoke a printer
    match args.output {
        Some(Output::HTML) => html::print(lex(args.input.unwrap()), args.theme.unwrap()),
        _ => term::print(lex(args.input.unwrap()), args.theme.unwrap()),
    }
    std::process::exit(0);
}
