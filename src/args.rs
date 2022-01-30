use structopt::StructOpt;
use crate::Error;

pub(crate) fn run() -> Result<(), Error> {
    println!("NI Extractor v0.0.1\n");

    match Command::from_args() {
        Command::Add{ description } => {
            let sw = stopwatch::Stopwatch::start_new();
            // do something that takes some time
            println!("{} took {}ms", description, sw.elapsed_ms());
        },
        Command::List => {},
    }

    Ok(())
}

#[derive(StructOpt)]
#[structopt()]
enum Command {
    Add {
        description: String
    },
    List,
}