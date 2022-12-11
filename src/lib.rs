use printer::{Output, ThemeKind};

pub mod lexer;
pub mod printer;

// what CLI arguments should look like
#[derive(Debug)]
pub struct Args {
    pub input: Option<String>,
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
        input: match pargs.opt_value_from_str("--input") {
            Ok(a) => a,
            Err(e) => return Err(e),
        },
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

// help print
pub static HELP: &'static str = include_str!("../README.txt");
