use pttypter::lexer::lex;
use pttypter::printer::{html, term, Output, ThemeKind};

const HELP: &'static str = r#"
pttypter
Marek Smolík, 2022
Simple CLI app to pretty-print JSON

USAGE:
  app [OPTIONS]

FLAGS:
  -h, --help             Prints help information

OPTIONS:
  --input  STRING        The input JSON code
  --theme  STRING        dark/light [DEFAULT: dark]
  --output STRING        html/term  [DEFAULT: term]
"#;

#[derive(Debug)]
struct Args {
    input: String,
    output: Option<Output>,
    theme: Option<ThemeKind>,
}

fn main() {
    let args = parse_args().unwrap_or_else(|e| {
        println!("Error: {e}.");
        std::process::exit(1);
    });

    let lexed = lex(args.input);
    match args.output {
        Some(Output::HTML) => html::print(lexed, args.theme.unwrap()),
        _ => term::print(lexed, args.theme.unwrap()),
    }
    std::process::exit(0);
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    if pargs.contains(["-h", "--help"]) {
        print!("{HELP}");
        std::process::exit(0);
    }

    let args = Args {
        input: pargs.value_from_str("--input")?,
        theme: Some(
            pargs
                .opt_value_from_str("--theme")?
                .unwrap_or(ThemeKind::Light),
        ),
        output: Some(
            pargs
                .opt_value_from_str("--output")?
                .unwrap_or(Output::Terminal),
        ),
    };

    Ok(args)
}
