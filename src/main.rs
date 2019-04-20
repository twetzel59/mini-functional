use std::{
    env::{self, Args},
    error::Error,
    fmt::{self, Display, Formatter},
};
use mini_functional::parsing::Parser;

#[derive(Clone, Copy, Debug)]
enum ArgErr {
    IncorrectNum,
}

impl Display for ArgErr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", match *self {
            ArgErr::IncorrectNum => "Incorrect number of arguments",
        })
    }
}

impl Error for ArgErr {}

/// Returns either Ok(filepath: String) or an Err(ArgErr)
fn handle_args(arguments: Args) -> Result<String, ArgErr> {
    let mut args = arguments.collect::<Vec<String>>();

    if args.len() == 2 {
        Ok(args.remove(1))
    } else {
        Err(ArgErr::IncorrectNum)
    }
}

fn main() -> Result<(), Box<Error>> {
    let path = match handle_args(env::args()) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("{}", e.to_string());
            return Err(e.into());
        },
    };

    let _parser = Parser::with_path(&path)?;
    //parser.test()?;

    Ok(())
}
