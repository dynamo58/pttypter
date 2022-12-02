use printer::{Output, ThemeKind};

pub mod lexer;
pub mod printer;

#[derive(Debug)]
pub struct Args {
    pub input: String,
    pub output: Option<Output>,
    pub theme: Option<ThemeKind>,
}

pub fn parse_args() -> Result<Args, pico_args::Error> {
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
                .unwrap_or(ThemeKind::Dark),
        ),
        output: Some(
            pargs
                .opt_value_from_str("--output")?
                .unwrap_or(Output::Terminal),
        ),
    };

    Ok(args)
}

pub static HELP: &'static str = r#"
pttypter
Marek Smolik, 2022
Simple CLI app to pretty-print JSON

USAGE:
  pttypter [OPTIONS]

FLAGS:
  -h, --help             Prints help information

OPTIONS:
  --input  STRING        The input JSON code
  --theme  STRING        dark/light [DEFAULT: dark]
  --output STRING        html/term  [DEFAULT: term]
"#;
